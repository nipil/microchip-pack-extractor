use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs;

const INDEX_NAME: &str = "index.idx";

pub async fn get(base_url: &str, client: &reqwest::Client) -> Index {
    fn parse_index(xml: &str) -> Index {
        let index: Index = from_str(&xml).expect("Index XML should parse correctly");
        eprintln!("Index has {} PDSC", index.pdscs.len());
        index
    }

    let url = format!("{base_url}/{INDEX_NAME}");

    // latest etag
    eprintln!("Fetching index header...");
    let etag = client
        .head(&url)
        .send()
        .await
        .expect("Cannot progress without getting latest index information");
    let etag = etag
        .headers()
        .get("ETag")
        .expect("Index headers must contain an ETag")
        .to_str()
        .expect("ETag must be convertible to string");
    let etag = quoted_string::strip_dquotes(etag).expect("Etag must be quoted");
    eprintln!("Most recent index header Etag is {etag}");

    // return local cache if any is available
    let cache = format!("{etag}.{INDEX_NAME}");
    if let Ok(content) = fs::read_to_string(&cache) {
        eprintln!("Reusing content from cache {cache}");
        return parse_index(&content);
    }

    // get latest content
    eprintln!("Fetching index content...");
    let res = client
        .get(url)
        .send()
        .await
        .expect("Cannot progress without getting latest index information");
    let etag = res
        .headers()
        .get("ETag")
        .expect("Index headers must contain an ETag")
        .to_str()
        .expect("ETag must be convertible to string");
    let etag = quoted_string::strip_dquotes(etag).expect("Etag must be quoted");
    let cache = format!("{}.{INDEX_NAME}", etag);
    let content = res.text().await.expect("Index must have content");

    // persist to local cache
    eprintln!("Writing content to cache {cache} for later use");
    eprintln!("Index string size is {}", content.len());
    fs::write(cache, &content).expect("Writing to cache must not fail");

    parse_index(&content)
}

#[derive(Debug, Deserialize)]
#[serde(rename = "idx")]
pub struct Index {
    #[serde(rename = "pdsc")]
    pdscs: Vec<Pdsc>,
}

#[derive(Debug, Deserialize)]
struct Pdsc {
    #[serde(rename = "@url")]
    url: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: String, // FIXME: semver ?
    #[serde(rename = "@atmel:name")]
    atmel_name: String,
    #[serde(rename = "@atmel:name.gz")]
    atmel_name_gz: String,
    #[serde(rename = "atmel:releases")]
    atmel_releases: AtmelReleases,
    #[serde(default, rename = "atmel:devices")]
    atmel_devices: Option<AtmelDevices>,
}

#[derive(Debug, Deserialize)]
struct AtmelReleases {
    #[serde(rename = "atmel:release")]
    atmel_release: Vec<AtmelRelease>,
}

#[derive(Debug, Deserialize)]
struct AtmelRelease {
    #[serde(rename = "@version")]
    version: String, // FIXME: semver
    #[serde(rename = "@date")]
    date: String, // FIXME: date
    #[serde(rename = "atmel:devices")]
    atmel_devices: AtmelDevices,
    // TODO: atmel:description
    // TODO: atmel:keywords
    // TODO: atmel:conditions
    // TODO: atmel:components
}

#[derive(Debug, Deserialize)]
struct AtmelDevices {
    #[serde(default, rename = "atmel:device")]
    atmel_device: Vec<AtmelDevice>,
}

#[derive(Debug, Deserialize)]
struct AtmelDevice {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@family")]
    family: String,
    #[serde(rename = "@core")]
    core: Option<String>,
    #[serde(rename = "@endian")]
    endian: Option<String>,
    // TODO: atmel:book
    // TODO: atmel:prerequisite
    // TODO: atmel:toolchain
}
