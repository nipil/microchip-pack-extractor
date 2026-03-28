use log::info;
use microchip_pack_extractor as mpe;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up !");
    let client = reqwest::Client::new();
    let index = mpe::pack_index(&client).await;
    index.process_dfps(&client).await;
}
