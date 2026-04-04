use quoted_string::strip_dquotes;
use reqwest::{Client, Response};
use tracing::{Level, debug, error, info, instrument, span, trace, warn};

pub async fn head_url(client: &Client, url: &str) -> Response {
    debug!(url = url, "Fetching headers");
    client
        .head(url)
        .send()
        .await
        .expect("Answer needed for HEAD {url}")
        .error_for_status()
        .expect("Failed to HEAD {url}")
}

pub async fn get_url(client: &Client, url: &str) -> Response {
    debug!(url = url, "Fetching content");
    client
        .get(url)
        .send()
        .await
        .expect("Answer needed for GET {url}")
        .error_for_status()
        .expect("Failed to GET {url}")
}

pub fn get_etag_from_response(res: &Response) -> &str {
    let etag = res
        .headers()
        .get("ETag")
        .expect("Headers must contain an ETag")
        .to_str()
        .expect("ETag must be convertible to string");
    trace!(etag = etag, "Found ETag in headers");
    strip_dquotes(etag).expect("Etag must be quoted")
}
