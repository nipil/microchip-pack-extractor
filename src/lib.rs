use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs;

const BASE_URL: &str = "https://packs.download.microchip.com";
const INDEX_NAME: &str = "index.idx";

async fn head_url(client: &reqwest::Client, url: &str) -> reqwest::Response {
    eprintln!("Fetching headers for {url}");
    client
        .head(url)
        .send()
        .await
        .expect("Cannot progress without head'ing {url}")
}

async fn get_url(client: &reqwest::Client, url: &str) -> reqwest::Response {
    eprintln!("Fetching content for {url}");
    client
        .get(url)
        .send()
        .await
        .expect("Cannot progress without get'ing {url}")
}

fn get_etag_from_response(res: &reqwest::Response) -> &str {
    let etag = res
        .headers()
        .get("ETag")
        .expect("Index headers must contain an ETag")
        .to_str()
        .expect("ETag must be convertible to string");
    quoted_string::strip_dquotes(etag).expect("Etag must be quoted")
}

fn get_cached_etag_bytes(name: &str, etag: &str) -> Option<Vec<u8>> {
    let cache = format!("{etag}.{name}");
    if let Ok(content) = fs::read(&cache) {
        eprintln!("Reusing cached content from {cache}");
        return Some(content);
    }
    eprintln!("No cache available for {cache}");
    None
}

fn set_cached_etag_bytes(name: &str, etag: &str, content: &Vec<u8>) {
    let cache = format!("{etag}.{name}");
    fs::write(cache, &content).expect("Writing to cache must not fail");
}

pub async fn get_cached_url_content_by_etag(
    client: &reqwest::Client,
    url: &str,
    cache_file: &str,
) -> Vec<u8> {
    // detect newest version using ETag
    let res = head_url(client, url).await;
    let etag = get_etag_from_response(&res);
    eprintln!("Most recent header Etag for {url} is {etag}");

    // return local cache if any is available
    let cache = get_cached_etag_bytes(cache_file, etag);
    if let Some(content) = cache {
        return content;
    }

    // get latest content
    let res = get_url(client, url).await;
    let etag = String::from(get_etag_from_response(&res));
    let content = res.bytes().await.expect("Index must have content");
    let content = Vec::from(content);

    // save content to cache
    set_cached_etag_bytes(cache_file, &etag, &content);

    content
}

pub async fn pack_index(client: &reqwest::Client) -> Idx {
    let url = format!("{BASE_URL}/{INDEX_NAME}");
    let content = get_cached_url_content_by_etag(client, &url, INDEX_NAME).await;
    let content = str::from_utf8(&content).expect("Index should be valid utf-8 text");
    from_str(&content).expect("Index XML should parse correctly")
}

#[derive(Deserialize)]
pub struct Idx {
    #[serde(rename = "pdsc")]
    pdscs: Vec<Pdsc>,
}

impl Idx {
    pub fn dpf_pdsc(&self) -> Vec<&Pdsc> {
        self.pdscs
            .iter()
            .filter(|x| x.name.ends_with("_DFP.pdsc"))
            .collect()
    }
}

#[derive(Deserialize)]
pub struct Pdsc {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String,
}

impl Pdsc {
    pub fn atpack_name(&self) -> String {
        let name = self
            .name
            .strip_suffix(".pdsc")
            .expect("PDSC name must end with .pdsc");
        format!("{}.{}.atpack", name, self.version)
    }
}
