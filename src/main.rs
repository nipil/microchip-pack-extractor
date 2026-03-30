use reqwest::Client;
use index::Index;

mod index;
mod package;
mod webcache;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = Client::new();
    let index = Index::get(&client).await;
    index.process_pdscs().await;
}
