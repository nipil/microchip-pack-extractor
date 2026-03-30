use futures::{StreamExt, stream};
use log::{debug, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use super::package;
use super::webcache;

const INDEX_URL: &str = "https://packs.download.microchip.com/index.idx";

pub struct Index<'a> {
    client: &'a Client,
    idx: Idx,
}

impl<'a> Index<'a> {
    pub async fn get(client: &'a Client) -> Self {
        webcache::ensure_cache_folder_exists();
        let idx = Idx::get(&client).await;
        Self { client, idx }
    }

    pub async fn process_pdscs(&self) {
        let limit: usize = num_cpus::get_physical();
        info!("Processing using {limit} cpu");

        // FIXME: remove slice
        // let r = &self.pdscs[0..1];
        let r = &self.idx.pdscs[..];
        let buffer = stream::iter(r)
            .map(|pdsc| async move {
                // FIXME: split async fetch with blocking post-processing
                pdsc.process(&self.client).await;
            })
            .buffer_unordered(limit);

        buffer.for_each(|_| async move {}).await;
    }
}

#[derive(Deserialize, Serialize)]
struct Idx {
    #[serde(rename = "pdsc")]
    pub pdscs: Vec<Pdsc>,
}

impl Idx {
    pub async fn get(client: &Client) -> Self {
        let cache = webcache::get_cached_url_content_by_etag(client, INDEX_URL).await;
        let content = str::from_utf8(&cache.content).expect("Index should be valid utf-8 text");
        info!("Parsing Index...");
        let index: Self = serde_xml_rs::from_str(&content).expect("Index XML must deserialize");
        debug!("Re-serializing to discard unused stuff...");
        let content = serde_xml_rs::to_string(&index).expect("Index XML must serialize");
        webcache::save_to_cache_file(&cache.cache_file, content.as_str().as_bytes());
        debug!(size=index.pdscs.len(); "Index size");
        index
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

    async fn process(&self, client: &Client) {
        let url = self.atpack_url();
        info!(url=url.as_str(); "Getting pack ...");
        let cache = webcache::get_cached_url_content_by_etag(&client, url.as_str()).await;
        // FIXME: not async code, move to post processing ? into new tasks ? multithreading ?
        info!(cache=cache.cache_file.as_str(), size=cache.content.len(); "Processing pack ... ");
        package::proces_zip(&cache.content, cache.file_name.as_str());
        info!(name=cache.file_name.as_str(); "Finished pack");
    }
}
