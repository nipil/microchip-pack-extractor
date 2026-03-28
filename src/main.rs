// #![deny(warnings)]

use anyhow::Result;

const BASE_URL: &str = "https://packs.download.microchip.com";
const INDEX: &str = "index.idx";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // default client with connection pooling
    let client = reqwest::Client::new();

    // process pack index
    let url = format!("{BASE_URL}/{INDEX}");

    // getting etag
    eprintln!("HEAD {url:?}...");
    let res = client.head(url).send().await?;
    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    // let body = res.text().await?;
    Ok(())
}
