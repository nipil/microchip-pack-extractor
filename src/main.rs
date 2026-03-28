const BASE_URL: &str = "https://packs.download.microchip.com";

mod index;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let index = index::get(BASE_URL, &client).await;
    eprintln!("{:#?}", index);
}
