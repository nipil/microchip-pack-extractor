const BASE_URL: &str = "https://packs.download.microchip.com";
const INDEX: &str = "index.idx";

#[tokio::main]
async fn main() {
    // default client with connection pooling
    let client = reqwest::Client::new();

    // process pack index
    let url = format!("{BASE_URL}/{INDEX}");

    // getting head
    eprintln!("Fetching headers for {url}");
    let res = client.head(url).send().await;
    let res = res.expect("FIXME");
    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    // extracting etag
    let etag = res.headers().get("ETag");
    let etag = etag.expect("FIXME");
    let etag = etag.to_str();
    let etag = etag.expect("FIXME");
    let etag = quoted_string::strip_dquotes(etag);
    let etag = etag.expect("FIXME");
    eprintln!("ETag : {}\n", etag);

    // let body = res.text().await?;
}
