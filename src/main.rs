use futures::{StreamExt, stream};
use reqwest::Client;
use tracing_flame::FlameLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod cache;
mod index;
mod package;
mod web;

#[tokio::main]
async fn main() {
    // FlameLayer writes a folded-stacks file while the guard is alive.
    // The guard flushes + closes the file on drop (end of main).
    let (flame_layer, _guard) =
        FlameLayer::with_file("./tracing.folded").expect("Flame tracing must not fail");

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .with(flame_layer)
        .init();

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
