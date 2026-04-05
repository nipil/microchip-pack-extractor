use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace_span};
use url::Url;

use crate::cache::EtagWebCache;

const INDEX_URL: &str = "https://packs.download.microchip.com/index.idx";

#[derive(Deserialize, Serialize)]
pub struct Idx {
    #[serde(rename = "pdsc")]
    pdscs: Vec<Pdsc>,
}

impl Idx {
    pub async fn fetch(cache: &EtagWebCache<'_>) -> Self {
        let (content, etag) = cache.get(INDEX_URL).await;
        let index = Self::parse(content).await;
        debug!("Re-serializing to discard unused stuff...");
        let content = serde_xml_rs::to_string(&index).expect("Index XML must serialize");
        cache
            .put(INDEX_URL, &etag, content.as_str().as_bytes())
            .await;
        info!(size = index.pdscs.len(), "Pdsc in Index");
        index
    }

    async fn parse(content: Vec<u8>) -> Self {
        info!("Parsing Index...");
        let (send, recv) = tokio::sync::oneshot::channel();
        // parsing the xml can be long
        // so spawn a sync thread and wait for completion
        // waiting is done through the channel
        rayon::spawn(move || {
            let content = str::from_utf8(&content).expect("Index should be valid utf-8 text");
            let span = trace_span!("Parsing index").entered();
            let idx = serde_xml_rs::from_str(&content).expect("Index XML must deserialize");
            drop(span);
            send.send(idx)
                .unwrap_or_else(|_| panic!("Receiver dropped in Idx::parse"));
        });
        recv.await.expect("Panic in rayon::spawn in Idx::parse")
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
    pub name: String,
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

    pub async fn fetch(&self, cache: &EtagWebCache<'_>) -> (Vec<u8>, String) {
        let url = self.atpack_url();
        info!(url = url.as_str(), "Getting pack ...");
        let (content, _) = cache.get(url.as_str()).await;
        (content, self.name.clone())
    }
}
