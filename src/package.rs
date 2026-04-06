use serde::Deserialize;
use std::{
    io::{Cursor, Read},
    path::{Path, PathBuf},
};
use tracing::{debug, info, instrument, trace_span, warn};
use zip::{ZipArchive, result::ZipError};

const PACKAGE_CONTENT: &str = "package.content";
const DESIRED_RESOURCE_TYPE: &str = "pic";

pub struct PdscArchive {
    name: String,
    zip_archive: ZipArchive<Cursor<Vec<u8>>>,
}

impl PdscArchive {
    pub fn new(name: String, zip_content: Vec<u8>) -> Self {
        let cursor = Cursor::new(zip_content);
        let zip_archive = ZipArchive::new(cursor).expect("Pdsc content must be a valid ZIP");
        Self { name, zip_archive }
    }

    fn get_file_content(&mut self, file_name: &str) -> Option<Vec<u8>> {
        let _span =
            trace_span!("Reading file from zip", file = file_name, zip = self.name).entered();
        // find file in zip archive
        let mut zip_file = match self.zip_archive.by_name(file_name) {
            Ok(zip_file) => zip_file,
            Err(e) => {
                if let ZipError::FileNotFound = e {
                    debug!(
                        zip = self.name,
                        file = file_name,
                        "File not found in archive"
                    );
                }
                return None;
            }
        };
        // preallocate buffer for file content, then read from archive into buffer
        let mut package_content = Vec::with_capacity(
            usize::try_from(zip_file.size())
                .expect("Uncompressed file size must fit in cpu architecture"),
        );
        zip_file
            .read_to_end(&mut package_content)
            .expect("Must be able to read package content");
        Some(package_content)
    }

    fn get_package_content(&mut self) -> Option<Package> {
        let Some(content) = self.get_file_content(PACKAGE_CONTENT) else {
            return None;
        };
        let _span = trace_span!("Deserializing", file = PACKAGE_CONTENT, zip = self.name).entered();
        let content = str::from_utf8(&content).expect("must be valid utf-8 text");
        let package = serde_xml_rs::from_str(content).expect("XML must deserialize");
        Some(package)
    }

    #[instrument(skip(archive))]
    pub async fn process(mut archive: Self) {
        let (send, recv) = tokio::sync::oneshot::channel();
        rayon::spawn(move || {
            match archive.get_package_content() {
                Some(package) => package.process(&mut archive),
                None => {
                    warn!(
                        zip = archive.name,
                        "Skipping archive without {}", PACKAGE_CONTENT
                    );
                }
            };
            send.send(()).unwrap_or_else(|_| panic!("Receiver dropped"));
            info!(name = archive.name, "Finished pack");
        });
        recv.await.expect("PdscArchive::process must send");
    }
}

#[derive(Deserialize, Debug)]
pub struct Package {
    content: Content,
}

impl Package {
    fn process(&self, mut archive: &mut PdscArchive) {
        for resources in &self.content.resources {
            for resource in &resources.resources {
                if resource.type_ != DESIRED_RESOURCE_TYPE {
                    continue;
                }
                for includes in &resource.includes {
                    let mut p = PathBuf::from(&resource.subdir);
                    p.push(&includes.pattern);
                    let p = p.to_str().expect("zip file name must be valid str");
                    includes.process(&p, &mut archive);
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
struct Content {
    resources: Vec<Resources>,
}

#[derive(Deserialize, Debug)]
struct Resources {
    #[serde(rename = "resource")]
    resources: Vec<Resource>,
}

#[derive(Deserialize, Debug)]
struct Resource {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@subdir")]
    subdir: String,
    #[serde(default, rename = "includes")]
    includes: Vec<Includes>,
}

#[derive(Deserialize, Debug)]
struct Includes {
    #[serde(rename = "@pattern")]
    pattern: String,
}

impl Includes {
    fn process(&self, file_name: &str, archive: &mut PdscArchive) {
        let content = archive.get_file_content(file_name).expect(&format!(
            "File {} must be in zip {}",
            file_name, archive.name
        ));
        warn!("path {} size {}", file_name, content.len());
        // TODO: parse
    }
}
