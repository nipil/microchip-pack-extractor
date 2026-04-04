use futures::{StreamExt, stream};
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

    // generate futures for each pdsc (using a nice iter syntactic sugar to keep Idx opaque)
    stream::iter(&idx)
        // to prefetch the content (from cache or web)
        .map(|pdsc| async { pdsc.fetch(&client).await })
        // run at most N concurrent futures
        // process them in whichever order they finish first
        .buffer_unordered(num_cpus::get_physical())
        // then feed them to a new task
        .for_each(|cache| async move { package::process_cache_result(cache).await })
        // wait for all tasks to complete
        .await;
}
