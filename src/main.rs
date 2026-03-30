use reqwest::Client;

mod cache;
mod index;
mod package;
mod web;

#[tokio::main]
async fn main() {
    env_logger::init();
    cache::ensure_cache_folder_exists().await;
    let client = Client::new();
    let idx = index::Idx::get(&client).await;
    idx.process_pdscs(&client).await;
}
