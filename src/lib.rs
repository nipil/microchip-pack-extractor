use serde::Deserialize;
use serde_xml_rs::from_str;
use std::fs;

const BASE_URL: &str = "https://packs.download.microchip.com";

const INDEX_NAME: &str = "index.idx";

pub async fn get(client: &reqwest::Client) -> Idx {
    fn parse_index(xml: &str) -> Idx {
        from_str(&xml).expect("Index XML should parse correctly")
    }

    let url = format!("{BASE_URL}/{INDEX_NAME}");

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
    } else {
        eprintln!("No available cache {cache}");
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
