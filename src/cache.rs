use crate::web;
use hex::encode;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::env::current_dir;
use std::path::PathBuf;
use tokio::fs;
use tokio::io;
use tracing::{debug, trace_span};

/* CACHE DIR *******************************************************************************/

const DEFAULT_CACHE_DIR_SUBFOLDER: &str = "cache";

pub struct CacheDir {
    base_dir: PathBuf,
}

impl CacheDir {
    fn key_hash(key: &str) -> String {
        encode(Sha256::digest(key))
    }

    pub async fn new(base_dir: Option<PathBuf>) -> Self {
        let cache_dir = match base_dir {
            None => {
                let mut base_dir = current_dir().expect("Must be able to detect current directory");
                base_dir.push(DEFAULT_CACHE_DIR_SUBFOLDER);
                Self { base_dir }
            }
            Some(base_dir) => Self { base_dir },
        };
        cache_dir.ensure_cache_folder_exists().await;
        cache_dir
    }

    async fn ensure_cache_folder_exists(&self) {
        debug!(
            cache = self.base_dir.as_path().to_string_lossy().as_ref(),
            "Ensuring cache folder exists"
        );
        fs::DirBuilder::new()
            .recursive(true)
            .create(&self.base_dir)
            .await
            .expect("Cache directory should exist");
    }

    fn cache_path_for(&self, key: &str) -> PathBuf {
        let mut cache_path = self.base_dir.clone();
        cache_path.push(&key);
        cache_path
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let key = Self::key_hash(key);
        let cache_path = self.cache_path_for(&key);
        let cache_str = cache_path.to_string_lossy();
        let cache_str = cache_str.as_ref();

        let span = trace_span!("Reading from cache dir", cache = cache_str).entered();
        let res = fs::read(&cache_path).await;
        drop(span);

        match res {
            Ok(content) => {
                debug!(cache = cache_str, size = content.len(), "Reading cache");
                return Some(content);
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    debug!(cache = cache_str, "No cache available");
                    None
                }
                _ => panic!(
                    "Reading an existing cached entry from disk must not fail: {} for {}",
                    e, cache_str
                ),
            },
        }
    }

    pub async fn put(&self, key: &str, value: &[u8]) {
        let key = Self::key_hash(key);
        let cache_path = self.cache_path_for(&key);
        let cache_str = cache_path.to_string_lossy();
        let cache_str = cache_str.as_ref();
        let span = trace_span!(
            "Writing to cache dir",
            cache = cache_str,
            size = value.len()
        )
        .entered();
        let s = fs::write(&cache_path, value).await;
        let _s = s.expect("Writing to cache must not fail");
        drop(span);
    }
}

/* ETAG WEB CACHE *******************************************************************************/

pub struct EtagWebCache<'a> {
    client: &'a Client,
    cache_dir: &'a CacheDir,
}

impl<'a> EtagWebCache<'a> {
    pub fn new(client: &'a Client, cache_dir: &'a CacheDir) -> Self {
        Self { client, cache_dir }
    }

    fn web_key(url: &str, etag: &str) -> String {
        format!("{url}|{etag}")
    }

    pub async fn get(&self, url: &str) -> (Vec<u8>, String) {
        // lookup local cache after fetching etag from remote site using head
        let res = web::head_url(self.client, url).await;
        let etag = web::get_etag_from_response(&res).to_string();
        let cache = self.cache_dir.get(&Self::web_key(url, &etag)).await;
        // cache hit
        if let Some(value) = cache {
            return (value, etag);
        }
        // cache miss : get latest content using get and cache its content using the latest etag
        let res = web::get_url(self.client, url).await;
        let etag = web::get_etag_from_response(&res).to_string();
        let value = Vec::from(res.bytes().await.expect("Url must have content"));
        self.cache_dir.put(&Self::web_key(url, &etag), &value).await;
        (value, etag)
    }

    pub async fn put(&self, url: &str, etag: &str, value: &[u8]) {
        let key = Self::web_key(url, etag);
        self.cache_dir.put(&key, value).await;
    }
}
