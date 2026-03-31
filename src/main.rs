use futures::{StreamExt, stream};
use log::info;
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

    // parallel processing
    let limit: usize = num_cpus::get_physical();

    stream::iter(&idx)
        .map(|pdsc| async { pdsc.fetch(&client).await })
        .buffer_unordered(limit)
        .for_each(|cache| async move {
            // FIXME: blocking function : spin in a long blocking task ?
            package::proces_zip(&cache.content, &cache.file_name);
            info!(name=cache.file_name.as_str(); "Finished pack");
        })
        .await;
}
