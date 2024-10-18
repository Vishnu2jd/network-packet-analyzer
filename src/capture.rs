use anyhow::{anyhow, Result};
use pnet::datalink::{self, Channel::Ethernet};
use tokio::sync::mpsc::Sender;

pub async fn capture_packets(interface_name: String, tx: Sender<Vec<u8>>) -> Result<()> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .ok_or_else(|| anyhow!("Network interface not found"))?;

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => return Err(anyhow!("Unhandled channel type")),
        Err(e) => return Err(anyhow!("Error creating datalink channel: {}", e)),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                tx.send(packet.to_vec()).await?;
            }
            Err(e) => eprintln!("Error receiving packet: {}", e),
        }
    }
}
