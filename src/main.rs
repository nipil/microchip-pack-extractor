use reqwest::Client;

mod index;
mod package;
mod webcache;

#[tokio::main]
async fn main() {
    env_logger::init();
    webcache::ensure_cache_folder_exists().await;
    let client = Client::new();
    let idx = index::Idx::get(&client).await;
    idx.process_pdscs(&client).await;
}
