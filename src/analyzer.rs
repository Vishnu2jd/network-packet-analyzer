use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocol;

pub fn analyze_packet(packet: &[u8]) -> String {
    let eth_packet = EthernetPacket::new(packet).unwrap();
    let mut analysis = format!(
        "Ethernet: Src: {}, Dst: {}",
        eth_packet.get_source(),
        eth_packet.get_destination()
    );

    match eth_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            analysis.push_str("\nIPv4: ");
            if let Some(ipv4_packet) = Ipv4Packet::new(eth_packet.payload()) {
                analysis.push_str(&analyze_ipv4_packet(&ipv4_packet));
            }
        }
        EtherTypes::Ipv6 => {
            analysis.push_str("\nIPv6: ");
            if let Some(ipv6_packet) = Ipv6Packet::new(eth_packet.payload()) {
                analysis.push_str(&analyze_ipv6_packet(&ipv6_packet));
            }
        }
        _ => analysis.push_str("\nUnsupported EtherType"),
    }

    analysis
}

fn analyze_ipv4_packet(ip_packet: &Ipv4Packet) -> String {
    let mut analysis = format!(
        "Src: {}, Dst: {}",
        ip_packet.get_source(),
        ip_packet.get_destination()
    );

    analyze_transport_layer(ip_packet.get_next_level_protocol(), ip_packet.payload(), &mut analysis);

    analysis
}

fn analyze_ipv6_packet(ip_packet: &Ipv6Packet) -> String {
    let mut analysis = format!(
        "Src: {}, Dst: {}",
        ip_packet.get_source(),
        ip_packet.get_destination()
    );

    analyze_transport_layer(ip_packet.get_next_header(), ip_packet.payload(), &mut analysis);

    analysis
}

fn analyze_transport_layer(protocol: IpNextHeaderProtocol, payload: &[u8], analysis: &mut String) {
    match protocol {
        IpNextHeaderProtocols::Tcp => {
            if let Some(tcp_packet) = TcpPacket::new(payload) {
                analysis.push_str(&format!(
                    "\nTCP: Src Port: {}, Dst Port: {}",
                    tcp_packet.get_source(),
                    tcp_packet.get_destination()
                ));
            }
        }
        IpNextHeaderProtocols::Udp => {
            if let Some(udp_packet) = UdpPacket::new(payload) {
                analysis.push_str(&format!(
                    "\nUDP: Src Port: {}, Dst Port: {}",
                    udp_packet.get_source(),
                    udp_packet.get_destination()
                ));
            }
        }
        _ => analysis.push_str("\nUnsupported IP Protocol"),
    }
}
