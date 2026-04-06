use futures::{StreamExt, stream};
use reqwest::Client;
use tracing_flame::FlameLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod cache;
mod index;
mod package;
mod pic;
mod web;

#[tokio::main]
async fn main() {
    // tracing configuration
    let (flame_layer, _guard) =
        FlameLayer::with_file("./tracing.folded").expect("Flame tracing must not fail");
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().with_span_events(fmt::format::FmtSpan::CLOSE))
        .with(flame_layer)
        .init();

    // dependency injection
    let cache_dir = cache::CacheDir::new(None).await;
    let client = Client::new();
    let web_cache = cache::EtagWebCache::new(&client, &cache_dir);

    // get the latest index
    let idx = index::Idx::fetch(&web_cache).await;

    // set some limits
    let limit_fetch = num_cpus::get_physical();
    let limit_processing = num_cpus::get();

    // process each pdsc using a nice iter syntactic sugar to keep Idx opaque
    stream::iter(&idx)
        // to prefetch the content (from cache or web)
        .map(|pdsc| async { pdsc.fetch(&web_cache).await })
        // run at most N concurrent futures
        // process them in whichever order they finish first
        .buffer_unordered(limit_fetch)
        // then feed them to a new task
        .for_each_concurrent(limit_processing, |archive| async move {
            package::PdscArchive::process(archive).await;
        })
        // wait for all tasks to complete
        .await;
}
