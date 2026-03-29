use clap::{Parser, Subcommand};
use microchip_pack_extractor as mpe;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Fetch,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    match args.command {
        Commands::Fetch => {
            let client = reqwest::Client::new();
            let index = mpe::pack_index(&client).await;
            index.process(&client).await;
        }
    }
}
