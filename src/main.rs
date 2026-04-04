use futures::{StreamExt, stream};
use reqwest::Client;
use tracing::instrument;

mod cache;
mod index;
mod package;
mod web;

#[instrument]
#[tokio::main]
async fn main() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Tracing subscriber must not fail");

    // Prepare resources for dependency injection
    cache::ensure_cache_folder_exists().await;
    let client = Client::new();
    let idx = index::Idx::get(&client).await;

    // process each pdsc using a nice iter syntactic sugar to keep Idx opaque
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
