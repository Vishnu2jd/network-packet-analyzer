mod analyzer;
mod capture;
mod cli;
mod utils;

use anyhow::Result;
use cli::Cli;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let interface = cli.interface.unwrap_or_else(|| utils::get_default_interface());

    println!("Starting packet capture on interface: {}", interface);

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    tokio::spawn(async move {
        capture::capture_packets(interface, tx).await.unwrap();
    });

    while let Some(packet) = rx.recv().await {
        let analysis = analyzer::analyze_packet(&packet);
        println!("{}", analysis);
    }

    Ok(())
}
