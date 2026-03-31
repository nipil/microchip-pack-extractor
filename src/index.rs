use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::cache;
use crate::cache::CacheResult;

const INDEX_URL: &str = "https://packs.download.microchip.com/index.idx";

#[derive(Deserialize, Serialize)]
pub struct Idx {
    #[serde(rename = "pdsc")]
    pdscs: Vec<Pdsc>,
}

impl Idx {
    pub async fn get(client: &Client) -> Self {
        let cache = cache::get_cached_url_content_by_etag(client, INDEX_URL).await;
        let content = str::from_utf8(&cache.content).expect("Index should be valid utf-8 text");
        info!("Parsing Index...");
        let index: Self = serde_xml_rs::from_str(&content).expect("Index XML must deserialize");
        debug!("Re-serializing to discard unused stuff...");
        let content = serde_xml_rs::to_string(&index).expect("Index XML must serialize");
        cache::save_to_cache_file(&cache.cache_file, content.as_str().as_bytes()).await;
        debug!(size=index.pdscs.len(); "Index size");
        index
    }
}

// chatgpt helped me with that, we're doomed.
impl<'a> IntoIterator for &'a Idx {
    type Item = &'a Pdsc;
    type IntoIter = std::slice::Iter<'a, Pdsc>;

    fn into_iter(self) -> Self::IntoIter {
        self.pdscs.iter()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Pdsc {
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

    pub async fn fetch(&self, client: &Client) -> CacheResult {
        let url = self.atpack_url();
        info!(url=url.as_str(); "Getting pack ...");
        cache::get_cached_url_content_by_etag(&client, url.as_str()).await
    }
}
