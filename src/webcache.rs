use log::{debug, trace};
use quoted_string::strip_dquotes;
use reqwest::{Client, Response};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use url::Url;

const CACHE_DIR: &str = "cache";

async fn head_url(client: &Client, url: &str) -> Response {
    debug!(url=url; "Fetching headers");
    client
        .head(url)
        .send()
        .await
        .expect("Answer needed for HEAD {url}")
        .error_for_status()
        .expect("Failed to HEAD {url}")
}

async fn get_url(client: &Client, url: &str) -> Response {
    debug!(url=url; "Fetching content");
    client
        .get(url)
        .send()
        .await
        .expect("Answer needed for GET {url}")
        .error_for_status()
        .expect("Failed to GET {url}")
}

fn get_etag_from_response(res: &Response) -> &str {
    let etag = res
        .headers()
        .get("ETag")
        .expect("Headers must contain an ETag")
        .to_str()
        .expect("ETag must be convertible to string");
    trace!(etag=etag; "Found ETag in headers");
    strip_dquotes(etag).expect("Etag must be quoted")
}

fn get_cache_dir() -> PathBuf {
    let mut p = current_dir().expect("Must be able to detect current directory");
    p.push(CACHE_DIR);
    p
}

pub fn ensure_cache_folder_exists() {
    let cache_dir = get_cache_dir();
    debug!(cache=cache_dir.as_path().to_string_lossy().as_ref(); "Ensuring cache folder exists");
    fs::DirBuilder::new()
        .recursive(true)
        .create(cache_dir)
        .expect("Cache directory should exist");
}

pub struct CacheResult {
    pub file_name: String,
    pub cache_file: String,
    pub content: Vec<u8>,
}

fn get_cache_file(file_name: &str, etag: &str) -> String {
    format!("{etag}.{file_name}")
}

pub async fn get_cached_url_content_by_etag(client: &Client, url: &str) -> CacheResult {
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
        return CacheResult {
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
    let content = res.bytes().await.expect("Url must have content");
    let content = Vec::from(content);

    // save content to cache
    save_to_cache_file(&cache_file, &content);

    CacheResult {
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

pub fn save_to_cache_file(cache_file: &str, content: &[u8]) {
    let mut cache = get_cache_dir();
    cache.push(&cache_file);
    let cache_str = cache.to_string_lossy();
    debug!(cache=cache_str.as_ref(), size=content.len(); "Writing cache");
    fs::write(&cache, content).expect("Writing to cache must not fail");
}
