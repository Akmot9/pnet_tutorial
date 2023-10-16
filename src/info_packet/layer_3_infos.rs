use pnet::packet::{
    ethernet::{EthernetPacket, EtherTypes}, 
    ipv6::Ipv6Packet, 
    ipv4::Ipv4Packet, 
    arp::ArpPacket, Packet};


// Define the Layer3Infos struct
pub struct Layer3Infos {
    pub ip_source: String,
    pub ip_destination: String,
}

pub fn get_layer_3_infos(ethernet_packet: &EthernetPacket<'_>) -> Layer3Infos{
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
            Layer3Infos {
                ip_source: String::from("N/A"),
                ip_destination: String::from("N/A"),
            }
        }
    }
}

struct Ipv4Handler;
struct Ipv6Handler;
struct ArpHandler;

trait HandlePacket {
    fn handle_packet(data: &[u8]) -> Layer3Infos;
}

impl HandlePacket for Ipv4Handler {
    fn handle_packet(data: &[u8]) -> Layer3Infos{
        if let Some(ipv4_packet) = Ipv4Packet::new(data) {
            println!(
                "Layer 3: IPv4 packet: source {} destination {} => {} {}",
                ipv4_packet.get_source(),
                ipv4_packet.get_destination(),
                ipv4_packet.get_next_level_protocol(),
                ipv4_packet.get_total_length()
            );
            //handle_next_proto_util(data, ipv4_packet.get_next_level_protocol());
            Layer3Infos { 
                ip_source: ipv4_packet.get_source().to_string(), 
                ip_destination: ipv4_packet.get_destination().to_string() 
            }
        } else {
            // Handle the case when the data is not a valid IPv4 packet
            Layer3Infos {
                ip_source: String::from("N/A"),
                ip_destination: String::from("N/A"),
            }
        }
    }
}



impl HandlePacket for Ipv6Handler {
    fn handle_packet(data: &[u8]) -> Layer3Infos{
        if let Some(ipv6_packet) = Ipv6Packet::new(data) {
            println!(
                "Layer 3: IPv6 packet: source {} destination {} => {} {}",
                ipv6_packet.get_source(),
                ipv6_packet.get_destination(),
                ipv6_packet.get_next_header(),
                ipv6_packet.get_payload_length()
            );
            Layer3Infos { 
                ip_source: ipv6_packet.get_source().to_string(), 
                ip_destination: ipv6_packet.get_destination().to_string() 
            }
            //handle_next_proto_util(data, ipv6_packet.get_next_header());
        } else {
            // Handle the case when the data is not a valid IPv4 packet
            Layer3Infos {
                ip_source: String::from("N/A"),
                ip_destination: String::from("N/A"),
            }
        }
    }
}

impl HandlePacket for ArpHandler {
    fn handle_packet(data: &[u8]) -> Layer3Infos{
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
            Layer3Infos { 
                ip_source: arp_packet.get_sender_proto_addr().to_string(), 
                ip_destination: arp_packet.get_target_proto_addr().to_string() 
            }
        } else {
            // Handle the case when the data is not a valid IPv4 packet
            Layer3Infos {
                ip_source: String::from("N/A"),
                ip_destination: String::from("N/A"),
            }
        }
    }
}