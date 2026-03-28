use std::fs;

const BASE_URL: &str = "https://packs.download.microchip.com";
const INDEX_NAME: &str = "index.idx";

async fn get_index(client: &reqwest::Client) -> String {
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
        return content;
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
    let etag = String::from(quoted_string::strip_dquotes(etag).expect("Etag must be quoted"));
    let content = res.text().await.expect("Index must have content");

    // persist to local cache
    let cache = format!("{etag}.{INDEX_NAME}");
    eprintln!("Writing content to cache {cache} for later use");
    fs::write(cache, &content).expect("Writing to cache must not fail");

    content
}

use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let index = get_index(&client).await;
    eprintln!("Index string size is {}", index.len());

    let src = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
    let should_be = Item {
        name: "Banana".to_string(),
        source: "Store".to_string(),
    };

    let item: Item = from_str(src).unwrap();
    assert_eq!(item, should_be);
}
