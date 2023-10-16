## Traits implementaion 

```rust
match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv6 => {
            if let Some(ipv6_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
                println!(
                    "Layer 3: IPv6 packet: source {} destination {} => {} {}",
                    ipv6_packet.get_source(),
                    ipv6_packet.get_destination(),
                    ipv6_packet.get_next_header(),
                    ipv6_packet.get_payload_length()
                );
            }
        },
        EtherTypes::Ipv4 => {
            if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                println!(
                    "Layer 3: IPv4 packet: source {} destination {} => {} {}",
                    ipv4_packet.get_source(),
                    ipv4_packet.get_destination(),
                    ipv4_packet.get_next_level_protocol(),
                    ipv4_packet.get_total_length(),
                );
            }
        },
        EtherTypes::Arp => {
            if let Some(arp_packet) = ArpPacket::new(ethernet_packet.payload()) {
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
        },
        _ => {
            // General case for all other EtherTypes
            println!(
                "Unknown or unsupported packet type: {:?}",
                ethernet_packet.get_ethertype()
            );
        }
    }
```
We can see that some of the ethertypes share some common behaviours. 