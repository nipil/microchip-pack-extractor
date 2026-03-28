use microchip_pack_extractor as mpe;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let index = mpe::get(&client).await;
    let dfps = index.dpf_pdsc();
    eprintln!("Found {} device packs", dfps.len());
    for dfp in dfps {
        eprintln!("{}", dfp.atpack_name());
    }
}
