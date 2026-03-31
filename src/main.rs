use futures::{StreamExt, stream};
use log::info;
use reqwest::Client;
use tokio::task::spawn_blocking;

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
        .for_each(|cache| async move {
            // which will be spawned on tokio thread pool for blocking operations
            let blocking_task = spawn_blocking(move || {
                package::proces_zip(&cache.content, &cache.file_name);
                info!(name=cache.file_name.as_str(); "Finished pack");
            });
            // wait for the tokio task wrapping the blocking operation to finish
            blocking_task
                .await
                .expect("Package process zip must not fail");
        })
        // wait for all tasks to complete
        .await;
}
