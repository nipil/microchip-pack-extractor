use futures::StreamExt;
use log::{debug, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::{env, fs, io, path};
use url::Url;

const CACHE_DIR: &str = "cache";
const INDEX_URL: &str = "https://packs.download.microchip.com/index.idx";

#[tokio::main]
async fn main() {
    env_logger::init();
    ensure_cache_folder_exists();
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

fn get_cache_dir() -> path::PathBuf {
    let mut p = env::current_dir().expect("Must be able to detect current directory");
    p.push(CACHE_DIR);
    p
}

fn ensure_cache_folder_exists() {
    let cache_dir = get_cache_dir();
    debug!(cache=cache_dir.as_path().to_string_lossy().as_ref(); "Ensuring cache folder exists");
    fs::DirBuilder::new()
        .recursive(true)
        .create(cache_dir)
        .expect("Cache directory should exist");
}

struct CacheItem {
    file_name: String,
    cache_file: String,
    content: Vec<u8>,
}

fn get_cache_file(file_name: &str, etag: &str) -> String {
    format!("{etag}.{file_name}")
}

async fn get_cached_url_content_by_etag(client: &reqwest::Client, url: &str) -> CacheItem {
    let url = Url::parse(url).expect("Must provide a valid url");
    let file_name = url
        .path_segments()
        .expect("Url must have a path")
        .last()
        .expect("Url must hold a filename")
        .to_string();

    // detect newest version using ETag
    let res = head_url(client, url.as_str()).await;

    // get from cache if it exists
    let cache_file = get_cache_file(&file_name, get_etag_from_response(&res));
    if let Some(content) = maybe_load_from_cache_file(&cache_file) {
        return CacheItem {
            file_name,
            cache_file,
            content,
        };
    }

    // TOC/TOU: ETag might changed inbetween requests
    drop(cache_file);

    // get latest content
    let res = get_url(client, url.as_str()).await;
    let cache_file = get_cache_file(&file_name, get_etag_from_response(&res));
    let content = res.bytes().await.expect("Index must have content");
    let content = Vec::from(content);

    // save content to cache
    save_to_cache_file(&cache_file, &content);

    CacheItem {
        file_name,
        cache_file,
        content,
    }
}

fn maybe_load_from_cache_file(cache_file: &str) -> Option<Vec<u8>> {
    let mut cache = get_cache_dir();
    cache.push(&cache_file);
    let cache_str = cache.to_string_lossy();
    debug!(cache=cache_str.as_ref(); "Cache candidate");
    if let Ok(content) = fs::read(&cache) {
        debug!(cache=cache_str.as_ref(), size=content.len(); "Reading cache");
        return Some(content);
    }
    debug!(cache=cache_str.as_ref(); "No cache available");
    None
}

fn save_to_cache_file(cache_file: &str, content: &[u8]) {
    let mut cache = get_cache_dir();
    cache.push(&cache_file);
    let cache_str = cache.to_string_lossy();
    debug!(cache=cache_str.as_ref(), size=content.len(); "Writing cache");
    fs::write(&cache, content).expect("Writing to cache must not fail");
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

#[derive(Deserialize, Serialize)]
struct Idx {
    #[serde(rename = "pdsc")]
    pdscs: Vec<Pdsc>,
}

impl Idx {
    async fn get(client: &reqwest::Client) -> Self {
        let cache = get_cached_url_content_by_etag(client, INDEX_URL).await;
        let content = str::from_utf8(&cache.content).expect("Index should be valid utf-8 text");
        info!("Parsing Index...");
        let index: Self = serde_xml_rs::from_str(&content).expect("Index XML must deserialize");
        debug!("Re-serializing to discard unused stuff...");
        let content = serde_xml_rs::to_string(&index).expect("Index XML must serialize");
        save_to_cache_file(&cache.cache_file, content.as_str().as_bytes());
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
struct Pdsc {
    #[serde(rename = "@url")]
    fqdn: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
}

impl Pdsc {
    fn atpack_url(&self) -> Url {
        let file_name = self
            .name
            .strip_suffix(".pdsc")
            .expect("PDSC name must end with .pdsc");
        let url = format!(
            "https://{}/{}.{}.atpack",
            self.fqdn, file_name, self.version
        );
        Url::parse(url.as_str()).expect("Must be a valid url")
    }

    async fn process(&self, client: &reqwest::Client) {
        let url = self.atpack_url();
        info!(url=url.as_str(); "Getting pack ...");
        let cache = get_cached_url_content_by_etag(&client, url.as_str()).await;
        info!(cache=cache.cache_file.as_str(), size=cache.content.len(); "Pack stored, processing ...");
        process_archive(&cache.content, cache.file_name.as_str());
        info!(name=cache.file_name.as_str(); "Finished pack");
    }
}
