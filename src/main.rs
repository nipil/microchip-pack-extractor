use futures::StreamExt;
use log::{debug, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::{fs, io};

const CACHE_DIR: &str = "cache";
const BASE_URL: &str = "https://packs.download.microchip.com";
const INDEX_NAME: &str = "index.idx";

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = reqwest::Client::new();
    let index = Idx::get(&client).await;
    index.process(&client).await;
}

async fn head_url(client: &reqwest::Client, url: &str) -> reqwest::Response {
    debug!(url=url; "Fetching headers");
    client
        .head(url)
        .send()
        .await
        .expect("Answer needed for HEAD {url}")
        .error_for_status()
        .expect("Failed to HEAD {url}")
}

async fn get_url(client: &reqwest::Client, url: &str) -> reqwest::Response {
    debug!(url=url; "Fetching content");
    client
        .get(url)
        .send()
        .await
        .expect("Answer needed for GET {url}")
        .error_for_status()
        .expect("Failed to GET {url}")
}

fn get_etag_from_response(res: &reqwest::Response) -> &str {
    let etag = res
        .headers()
        .get("ETag")
        .expect("Index headers must contain an ETag")
        .to_str()
        .expect("ETag must be convertible to string");
    trace!(etag=etag; "Found ETag in headers");
    quoted_string::strip_dquotes(etag).expect("Etag must be quoted")
}

fn ensure_cache_folder() {
    debug!(cache=CACHE_DIR; "Ensuring cache folder exists");
    fs::DirBuilder::new()
        .recursive(true)
        .create(CACHE_DIR)
        .expect("Cache directory should exist");
}

struct CacheItem {
    content: Vec<u8>,
    etag: String,
}

async fn get_cached_url_content_by_etag(
    client: &reqwest::Client,
    url: &str,
    file_name: &str,
) -> CacheItem {
    // detect newest version using ETag
    let res = head_url(client, url).await;
    let etag = get_etag_from_response(&res);
    debug!(url=url, etag=etag; "Lookup most recent etag for url");

    // ensure the cache folder exists
    ensure_cache_folder();

    // get from cache if it exists
    if let Some(content) = maybe_load_from_cache_file(file_name, etag) {
        return CacheItem {
            content,
            etag: etag.to_string(),
        };
    }

    // get latest content
    let res = get_url(client, url).await;
    let etag = String::from(get_etag_from_response(&res));
    let content = res.bytes().await.expect("Index must have content");
    let content = Vec::from(content);

    // save content to cache (TOC/TOU: ETag might changed inbetween)
    save_to_cache_file(file_name, &etag, &content);

    CacheItem { content, etag }
}

fn maybe_load_from_cache_file(file_name: &str, etag: &str) -> Option<Vec<u8>> {
    let cache_file = format!("{CACHE_DIR}/{etag}.{file_name}");
    trace!(cache=cache_file.as_str(); "Cache path");
    if let Ok(content) = fs::read(&cache_file) {
        debug!(cache=cache_file.as_str(), size=content.len(); "Reading cache");
        return Some(content);
    }
    debug!(cache=cache_file.as_str(); "No cache available");
    // return local cache if any is available
    None
}

fn save_to_cache_file(file_name: &str, etag: &str, content: &[u8]) {
    let cache_file = format!("{CACHE_DIR}/{etag}.{file_name}");
    debug!(cache=cache_file.as_str(), size=content.len(); "Writing cache");
    fs::write(cache_file, content).expect("Writing to cache must not fail");
}

fn process_archive(content: &[u8], zip_name: &str) {
    let content = io::Cursor::new(content);
    let mut zip = zip::ZipArchive::new(content).expect("Atpack must be a valid zip archive");
    for i in 0..zip.len() {
        let entry = zip.by_index(i).expect("Must be able to get entry from zip");
        let enclosed_name = entry
            .enclosed_name()
            .expect("Zip entry must have an enclosed name");
        let enclosed_name = enclosed_name
            .to_str()
            .expect("Zip entry must be valid utf-8");
        if entry.is_dir() {
            debug!(dir=enclosed_name, zip=zip_name; "Ignoring directory");
            continue;
        }
        if entry.is_symlink() {
            warn!(symlink=enclosed_name, zip=zip_name; "Ignoring symlink");
            continue;
        }
        if !entry.is_file() {
            panic!(
                "Unknown entry {} is not a file in {}",
                enclosed_name, zip_name
            );
        }
        eprintln!("name {} size {}", enclosed_name, entry.size());
    }
}

impl Idx {
    async fn get(client: &reqwest::Client) -> Self {
        let url = format!("{BASE_URL}/{INDEX_NAME}");
        let cache = get_cached_url_content_by_etag(client, &url, INDEX_NAME).await;
        let content = str::from_utf8(&cache.content).expect("Index should be valid utf-8 text");
        info!("Parsing Index...");
        let index: Self = serde_xml_rs::from_str(&content).expect("Index XML must deserialize");
        debug!("Re-serializing to discard unused stuff...");
        let content = serde_xml_rs::to_string(&index).expect("Index XML must serialize");
        save_to_cache_file(INDEX_NAME, &cache.etag, content.as_str().as_bytes());
        debug!(size=index.pdscs.len(); "Index size");
        index
    }

    async fn process(&self, client: &reqwest::Client) {
        let limit: usize = num_cpus::get_physical();
        info!("Processing using {limit} cpu");
        futures::stream::iter(&self.pdscs /* [0..1] */) // FIXME: remove slice
            .map(|pdsc| async move {
                pdsc.process(&client).await;
            })
            .buffer_unordered(limit)
            .for_each(|_| async move {})
            .await;
    }
}

#[derive(Deserialize, Serialize)]
struct Idx {
    #[serde(rename = "pdsc")]
    pdscs: Vec<Pdsc>,
}

#[derive(Deserialize, Serialize)]
struct Pdsc {
    #[serde(rename = "@url")]
    fqdn: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
}

struct Atpack {
    file_name: String,
    url: String,
}

impl Pdsc {
    fn atpack_info(&self) -> Atpack {
        let file_name = self
            .name
            .strip_suffix(".pdsc")
            .expect("PDSC name must end with .pdsc");
        let file_name = format!("{}.{}.atpack", file_name, self.version);
        let url = format!("https://{}/{}", self.fqdn, file_name);
        Atpack { file_name, url }
    }

    async fn process(&self, client: &reqwest::Client) {
        let atpack = self.atpack_info();
        info!(name=atpack.file_name.as_str(); "Getting pack ...");
        // fetch, read and cache
        let cache = get_cached_url_content_by_etag(&client, &atpack.url, &atpack.file_name).await;
        info!(name=atpack.file_name.as_str(), size=cache.content.len(), etag=cache.etag.as_str(); "Pack read, processing ...");
        // process interesting files in the archive
        process_archive(&cache.content, atpack.file_name.as_str());
        info!(name=atpack.file_name.as_str(); "Finished pack");
    }
}
