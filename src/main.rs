use microchip_pack_extractor as mpe;

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = reqwest::Client::new();
    let index = mpe::pack_index(&client).await;
    index.process_dfps(&client).await;
}
