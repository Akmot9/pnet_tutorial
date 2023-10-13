use pnet::packet::{ethernet::{EthernetPacket, EtherTypes}, 
                    arp::ArpPacket, 
                    ipv6::Ipv6Packet, 
                    ipv4::Ipv4Packet, 
                    Packet, 
                    ip::IpNextHeaderProtocol,
                    tcp::TcpPacket, 
                    udp::UdpPacket,
                };
trait HandlePacket {
    fn handle_packet(data: &[u8]);
}

pub trait HandleNextProto {
    fn handle_next_proto(&self, data: &[u8], proto: IpNextHeaderProtocol);
}

struct Ipv4Handler;
struct Ipv6Handler;
struct ArpHandler;

impl HandlePacket for Ipv4Handler {
    fn handle_packet(data: &[u8]) {
        if let Some(ipv4_packet) = Ipv4Packet::new(data) {
            println!(
                "Layer 3: IPv4 packet: source {} destination {} => {} {}",
                ipv4_packet.get_source(),
                ipv4_packet.get_destination(),
                ipv4_packet.get_next_level_protocol(),
                ipv4_packet.get_total_length()
            );
            handle_next_proto_util(data, ipv4_packet.get_next_level_protocol());
        }
    }
}

impl HandlePacket for Ipv6Handler {
    fn handle_packet(data: &[u8]) {
        if let Some(ipv6_packet) = Ipv6Packet::new(data) {
            println!(
                "Layer 3: IPv6 packet: source {} destination {} => {} {}",
                ipv6_packet.get_source(),
                ipv6_packet.get_destination(),
                ipv6_packet.get_next_header(),
                ipv6_packet.get_payload_length()
            );
            handle_next_proto_util(data, ipv6_packet.get_next_header());
        }
    }
}

impl HandleNextProto for Ipv4Handler {
    fn handle_next_proto(&self, data: &[u8], proto: IpNextHeaderProtocol) {
        handle_next_proto_util(data, proto);
    }
}

impl HandleNextProto for Ipv6Handler {
    fn handle_next_proto(&self, data: &[u8], proto: IpNextHeaderProtocol) {
        handle_next_proto_util(data, proto);
    }
}

pub fn handle_next_proto_util(data: &[u8], proto: IpNextHeaderProtocol) {
    match proto {
        pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
            if let Some(tcp_packet) = TcpPacket::new(data) {
                println!(
                    "    TCP Source port: {}, Destination port: {}",
                    tcp_packet.get_source(),
                    tcp_packet.get_destination()
                );
            }
        }
        pnet::packet::ip::IpNextHeaderProtocols::Udp => {
            if let Some(udp_packet) = UdpPacket::new(data) {
                println!(
                    "    Udp Source port: {}, Destination port: {}",
                    udp_packet.get_source(),
                    udp_packet.get_destination()
                );
            }
        }
        // Add other protocols here
        _ => {
            println!("    Unsupported next header protocol: {:?}", proto);
        }
    }
}

impl HandlePacket for ArpHandler {
    fn handle_packet(data: &[u8]) {
        if let Some(arp_packet) = ArpPacket::new(data) {
            println!(
                "Layer 2: arp packet: source {} destination {} => {:?} {} {} {:?} {}",
                arp_packet.get_sender_hw_addr(),
                arp_packet.get_target_hw_addr(),
                arp_packet.get_operation(),
                arp_packet.get_target_proto_addr(),
                arp_packet.get_sender_proto_addr(),
                arp_packet.get_hardware_type(),
                arp_packet.get_proto_addr_len()
            );
        }
    }
}

fn print_packet_layer_2(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    println!(
        "Layer 2: New packet on {}: {} => {}: {}",
        interface_name,
        ethernet_packet.get_source(),
        ethernet_packet.get_destination(),
        ethernet_packet.get_ethertype()
    );
}

pub fn process_packet_by_type(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    print_packet_layer_2(interface_name, ethernet_packet);
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv6 => Ipv6Handler::handle_packet(ethernet_packet.payload()),
        EtherTypes::Ipv4 => Ipv4Handler::handle_packet(ethernet_packet.payload()),
        EtherTypes::Arp => ArpHandler::handle_packet(ethernet_packet.payload()),
        _ => {
            // General case for all other EtherTypes
            println!(
                "Unknown or unsupported packet type: {:?}",
                ethernet_packet.get_ethertype()
            );
        }
    }
}