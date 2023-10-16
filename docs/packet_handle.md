# Rust Packet Protocol Handling Tutorial

Welcome to the Rust Packet Protocol Handling Tutorial! In this tutorial, you'll embark on a journey to learn how to handle packet protocols using Rust programming language. Packet protocol handling is a crucial aspect of networking and cybersecurity, making this knowledge highly valuable for software engineers and network enthusiasts.

We will see how to implement Traits and trait bounds to let us write code that uses generic type parameters to reduce duplication. 

## Prerequisites

Ensure you have reviewed the previous tutorials to stay up-to-date with the content. 

## 1. Current Codebase

this is now our src/main.rs file:

```rust
use pnet::datalink::Channel::Ethernet;
use pnet::datalink;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use pnet::packet::FromPacket;
use std::thread;

fn main() {
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, 
                                                                    Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}",&interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface);
        loop {
            match rx.next() {
                Ok(packet) => {
                    if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                        println!("New packet on {}", interface.name);
                        println!("{} => {}: {}",
                            ethernet_packet.get_destination(),
                            ethernet_packet.get_source(),
                            ethernet_packet.get_ethertype());
                        let packet = ethernet_packet.packet();
                        let payload = ethernet_packet.payload();
                        let from_packet = ethernet_packet.from_packet();
                        //println!("---");
                        println!("packet: {:?}", packet);
                        // print the full packet as an array of u8
                        println!("payload: {:?}", payload);
                        // print the payload as an array of u8
                        println!("from_packet: {:?}", from_packet);
                        // print the hearder infos: mac address, ethertype, ...
                        // and the payload as an array of u8
                        println!("---");
                        
                    }
                }
                Err(e)=> {
                    panic!("An error occurred while reading: {}", e);
                }
            }
        }
    }
}
```
The expected output is:
```bash
Start thread reading packet on interface: lo: flags=10049<UP,LOOPBACK,RUNNING,LOWERUP>
      index: 1
      ether: 00:00:00:00:00:00
       inet: 127.0.0.1/8
      inet6: ::1/128
New packet on enx0a87c76ee9f1
0a:87:c7:6e:e9:f1 => 0a:87:c7:e6:6f:64: Ipv6
packet: [10, 135, 199, 110, 233, 241, 10, 135, 199, 230, 111, 100, 134, 221, 96, 0, 0, 0, 0, 32, 58, 255, 42, 4, 206, 192, 17, 176, 30, 98, 133, 56, 164, 51, 110, 138, 232, 114, 42, 4, 206, 192, 17, 176, 30, 98, 94, 176, 89, 105, 39, 178, 168, 198, 135, 0, 41, 190, 0, 0, 0, 0, 42, 4, 206, 192, 17, 176, 30, 98, 94, 176, 89, 105, 39, 178, 168, 198, 1, 1, 10, 135, 199, 230, 111, 100]
payload: [96, 0, 0, 0, 0, 32, 58, 255, 42, 4, 206, 192, 17, 176, 30, 98, 133, 56, 164, 51, 110, 138, 232, 114, 42, 4, 206, 192, 17, 176, 30, 98, 94, 176, 89, 105, 39, 178, 168, 198, 135, 0, 41, 190, 0, 0, 0, 0, 42, 4, 206, 192, 17, 176, 30, 98, 94, 176, 89, 105, 39, 178, 168, 198, 1, 1, 10, 135, 199, 230, 111, 100]
from_packet: Ethernet { destination: 0a:87:c7:6e:e9:f1, source: 0a:87:c7:e6:6f:64, ethertype: EtherType(34525), payload: [96, 0, 0, 0, 0, 32, 58, 255, 42, 4, 206, 192, 17, 176, 30, 98, 133, 56, 164, 51, 110, 138, 232, 114, 42, 4, 206, 192, 17, 176, 30, 98, 94, 176, 89, 105, 39, 178, 168, 198, 135, 0, 41, 190, 0, 0, 0, 0, 42, 4, 206, 192, 17, 176, 30, 98, 94, 176, 89, 105, 39, 178, 168, 198, 1, 1, 10, 135, 199, 230, 111, 100] }
---
```


To achieve this organization, we'll encapsulate this logic in a separate file located at src/info_packet.rs. Let's put the layer 2 handler function in it for a start: 
```rust
use pnet::packet::ethernet::EthernetPacket;

pub fn print_packet_layer_2(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    println!(
        "Layer 2: New packet on {}: {} => {}: {}",
        interface_name,
        ethernet_packet.get_source(),
        ethernet_packet.get_destination(),
        ethernet_packet.get_ethertype()
    );
}
```
Please note that I made some adjustments to the displayed information. Additionally, we utilize the 'pub' keyword to make this functionality accessible from outside the module, as demonstrated in src/main.rs.
```rust
mod info_packet;

fn main() {
    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    info_packet::print_packet_layer_2(&interface.name, &ethernet_packet)
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
```
The expected output is now:
```bash
Start thread reading packet on interface: enx0a87c76ee9f1: flags=11043<UP,BROADCAST,MULTICAST,RUNNING,LOWERUP>
      index: 8
      ether: 0a:87:c7:6e:e9:f1
       inet: 172.20.10.8/28
      inet6: 2a04:cec0:11b0:1e62:5eb0:5969:27b2:a8c6/64
      inet6: 2a04:cec0:11b0:1e62:899f:d5ca:3247:3bfc/64
      inet6: fe80::dbea:67dc:4f2c:ed9/64
Start thread reading packet on interface: enx3c18a0bfb354: flags=1003<UP,BROADCAST,MULTICAST>
      index: 4
      ether: 3c:18:a0:bf:b3:54
---
Layer 2: New packet on enx0a87c76ee9f1: 0a:87:c7:6e:e9:f1 => 0a:87:c7:e6:6f:64: Ipv6
---
Layer 2: New packet on enx0a87c76ee9f1: 0a:87:c7:e6:6f:64 => 0a:87:c7:6e:e9:f1: Ipv6
---
Layer 2: New packet on enx0a87c76ee9f1: 0a:87:c7:6e:e9:f1 => 0a:87:c7:e6:6f:64: Ipv6

```

In summary, we've successfully separated the logic for printing packet layer 2 information into a separate module, info_packet.rs, and imported it into the main.rs file. This modular approach enhances code organization and maintainability. The code is now capable of printing out layer 2 information for packets received on different network interfaces.


## 2. Extracting Layer 3 Information

We have successfully captured Ethernet frames and obtained their Ethernet EtherTypes using `ethernet_packet.get_ethertype()`. Now, let's focus on handling the most common layer 3 protocols: IPv4, IPv6, and ARP.

As you saw in the Current Codebase part, the packets ans payloads are represented in a list of u8. Let's categorize them by their addresses and protocols following this format:

Layer 3: ip_source, layer_3_protocols, ip_dest, next_layer_protocol, length.

### Handling Layer 3 Protocols

The `get_ethertype()` function returns an `EtherType` value, which indicates the encapsulated protocol within the Ethernet frame. We can use this value to determine the layer 3 protocol. Here are the common layer 3 EtherTypes we may encounter:

- [EtherTypes Documentation](https://docs.rs/pnet/0.34.0/pnet/packet/ethernet/EtherTypes/index.html)

#### IPv4 (EtherType: 0x0800)

IPv4 is one of the most widely used layer 3 protocols on the Internet. To handle IPv4 packets, we can check if the EtherType is equal to `EtherTypes::Ipv4`. If it matches, we can proceed to extract IPv4-specific information.

```rust
if ethernet_packet.get_ethertype() == EtherTypes::Ipv4 {
    // Extract IPv4 information here
}
```

#### IPv6 (EtherType: 0x86DD)

IPv6 is the successor to IPv4 and is becoming increasingly prevalent. To handle IPv6 packets, we check if the EtherType is equal to `EtherTypes::Ipv6`.

```rust
if ethernet_packet.get_ethertype() == EtherTypes::Ipv6 {
    // Extract IPv6 information here
}
```

#### ARP (EtherType: 0x0806)

ARP (Address Resolution Protocol) is used to map IP addresses to MAC addresses on a local network. To handle ARP packets, we check if the EtherType is equal to `EtherTypes::Arp`.

```rust
if ethernet_packet.get_ethertype() == EtherTypes::Arp {
    // Extract ARP information here
}
```

While the if statements above work, using a match expression is a more idiomatic and structured way to handle multiple possibilities. Here's how we can refactor the code using a match expression:

```rust
match ethernet_packet.get_ethertype() {
    EtherTypes::Ipv4 => {
        // Handle IPv4 packets
        // Extract IPv4 information here
    }
    EtherTypes::Ipv6 => {
        // Handle IPv6 packets
        // Extract IPv6 information here
    }
    EtherTypes::Arp => {
        // Handle ARP packets
        // Extract ARP information here
    }
    _ => {
        // Handle other EtherTypes or unknown cases
    }
}
```

Using `match` provides better clarity and ensures that we explicitly cover all possible cases. It's a cleaner and more maintainable approach for handling different EtherTypes in Rust. Thank you for suggesting this improvement.

By following these conditional checks based on EtherTypes, we can branch our code to handle specific layer 3 protocols accordingly. In the subsequent sections of our tutorial, we can delve into how to extract and work with the information specific to each protocol. 

Look at the modifications in the info_packet.rs:
```rust
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};

pub fn process_packet_by_type(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    print_packet_layer_2(interface_name, ethernet_packet);
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv6 => {println!("ipv6")},
        EtherTypes::Ipv4 => {println!("ipv4")},
        EtherTypes::Arp => {println!("Arp")},
        _ => {
            // General case for all other EtherTypes
            println!(
                "Unknown or unsupported packet type: {:?}",
                ethernet_packet.get_ethertype()
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
```
for now we just print the EtherType so nothing very new. 
Let's get specific infos about each of them then : 

```rust
pub fn process_packet_by_type(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) {
    print_packet_layer_2(interface_name, ethernet_packet);
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
}
```

The expected output should look like this: 
```bash
---
Layer 2: New packet on lo: 00:00:00:00:00:00 => 00:00:00:00:00:00: Ipv4
Layer 3: IPv4 packet: source 127.0.0.53 destination 127.0.0.1 => Udp 134
---
Layer 2: New packet on enx0a87c76ee9f1: 0a:87:c7:6e:e9:f1 => 0a:87:c7:e6:6f:64: Ipv6
Layer 3: IPv6 packet: source 2a04:cec0:11b0:1e62:ad9:950f:1d37:c45e destination 2600:1901:0:4d00:: => Tcp 61

```