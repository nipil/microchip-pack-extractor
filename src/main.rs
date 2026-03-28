mod index;

const BASE_URL: &str = "https://packs.download.microchip.com";

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let index = index::get(BASE_URL, &client).await;
    let dfps = index.dpf_pdsc();
    eprintln!("Found {} device packs", dfps.len());
    for dfp in dfps {
        eprintln!("{}", dfp.atpack_name());
    }
}
