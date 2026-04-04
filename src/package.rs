use serde::Deserialize;
use std::io::{Cursor, Read};
use tracing::{Level, debug, error, info, instrument, span, trace, warn};
use zip::ZipArchive;

use crate::cache::CacheResult;

const PACKAGE_CONTENT: &str = "package.content";

pub async fn process_cache_result(cache: CacheResult) {
    // processing the zip will be long
    // spawn a sync thread
    // and wait for completion through async channel
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        proces_zip(&cache.content, &cache.file_name);
        send.send(())
            .unwrap_or_else(|_| panic!("Receiver dropped in Idx::parse"));
    });
    recv.await
        .expect("Panic in rayon::spawn : package processing of zip must not fail")
}

fn proces_zip(content: &[u8], zip_name: &str) {
    info!(name = zip_name, "Processing pack ... ");
    let content = Cursor::new(content);
    let mut zip = ZipArchive::new(content).expect("Atpack must be a valid zip file");
    let Ok(mut package_file) = zip.by_name(PACKAGE_CONTENT) else {
        warn!(
            zip = zip_name,
            "Skipping zip because no package content was found"
        );
        return;
    };

    let mut package_content: Vec<u8> = Vec::new();
    package_file
        .read_to_end(&mut package_content)
        .expect("Must be able to read package content");
    drop(package_file);

    info!(zip = zip_name, "Parsing package content...");
    let package_content =
        str::from_utf8(&package_content).expect("Package content should be valid utf-8 text");
    PackageContent::new(zip_name, &mut zip, package_content).process();
    info!(name = zip_name, "Finished pack");
}

struct PackageContent<'a, T> {
    zip_name: &'a str,
    zip: &'a mut ZipArchive<T>,
    package: Package,
}

impl<'a, T> PackageContent<'a, T> {
    fn new(zip_name: &'a str, zip: &'a mut ZipArchive<T>, content: &str) -> Self {
        Self {
            zip_name,
            zip,
            package: serde_xml_rs::from_str(content).expect("Package content XML must deserialize"),
        }
    }

    fn process(&self) {
        trace!("Package{:?}", self.package);
    }
}

#[derive(Deserialize, Debug)]
struct Package {
    content: Content,
}

#[derive(Deserialize, Debug)]
struct Content {
    resources: Vec<Resources>,
}

#[derive(Deserialize, Debug)]
struct Resources {
    #[serde(rename = "@target")]
    target: String,
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
    #[serde(default, rename = "meta")]
    meta: Vec<Meta>,
}

#[derive(Deserialize, Debug)]
struct Includes {
    #[serde(rename = "@pattern")]
    pattern: String,
}

#[derive(Deserialize, Debug)]
struct Meta {
    #[serde(rename = "@key")]
    key: String,
    #[serde(rename = "@value")]
    value: String,
}
