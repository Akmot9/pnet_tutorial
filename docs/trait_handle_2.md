## Title: Implementing Traits in Rust for Packet Handling with pnet - Part 2: Layer 3

### Introduction:

Welcome to the second part of our tutorial series on implementing traits in Rust for packet handling with the `pnet` library. In this installment, we'll build upon the foundation we established in Part 1, where we handled Layer 2 network packets. Now, we'll dive into handling Layer 3 network packets, including both IPv4 and IPv6 packets.

### Prerequisites:

- Basic knowledge of Rust.
- Familiarity with the `pnet` library for packet manipulation.
- Completion of Part 1 of this tutorial series where we introduced handling Layer 2 packets.

### Organizing Code into Modules:

As a reminder, organizing your code into modules is crucial for code maintainability and readability. In Part 1, we created a module named `info_packet` to encapsulate our packet handling functionality. We'll continue to use this modular approach in Part 2.

### Step 1: Defining the Layer3Infos Struct:

In Part 2, we introduce the `Layer3Infos` struct, which will hold information specific to Layer 3 network packets. This includes the source and destination IP addresses.

```rust
use pnet::packet::{
    ethernet::{EthernetPacket, EtherTypes},
    ipv6::Ipv6Packet,
    ipv4::Ipv4Packet,
    arp::ArpPacket,
    Packet,
};

pub struct Layer3Infos {
    pub ip_source: String,
    pub ip_destination: String,
}
```

The `Layer3Infos` struct is a critical part of our packet handling as it will store Layer 3-specific information.

### Step 2: Implementing the `HandlePacket` Trait:

In Part 2, we define the `HandlePacket` trait, which will be implemented by different packet handlers to extract Layer 3 information. Here's how the trait definition looks:

```rust
trait HandlePacket {
    fn get_layer_3(data: &[u8]) -> Layer3Infos;
}
```

The `HandlePacket` trait defines a method named `get_layer_3` that takes a reference to packet data and returns a `Layer3Infos` object. Different packet handlers, such as `Ipv4Handler`, `Ipv6Handler`, and `ArpHandler`, will implement this trait to extract Layer 3 information.

### Step 3: Implementing Packet Handlers for IPv4, IPv6, and ARP:

We've defined three packet handlers for IPv4, IPv6, and ARP packets: `Ipv4Handler`, `Ipv6Handler`, and `ArpHandler`. Each of these handlers implements the `HandlePacket` trait by providing an implementation for the `get_layer_3` method.

Here's an example of the `Ipv4Handler` implementation:

```rust
impl HandlePacket for Ipv4Handler {
    fn get_layer_3(data: &[u8]) -> Layer3Infos {
        // Implementation for handling IPv4 packets
        // Extract IP source and destination addresses
        // ...
        Layer3Infos {
            ip_source: source_address.to_string(),
            ip_destination: destination_address.to_string(),
        }
    }
}
```

Each handler performs specific packet parsing and returns a `Layer3Infos` object containing the source and destination IP addresses.

### Step 4: Implementing `get_layer_3_infos` Function:

We create a function named `get_layer_3_infos` that takes an `EthernetPacket` as input and determines its Layer 3 type (IPv4, IPv6, ARP). Depending on the type, it delegates to the appropriate packet handler to extract Layer 3 information.

```rust
pub fn get_layer_3_infos(ethernet_packet: &EthernetPacket<'_>) -> Layer3Infos {
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv6 => Ipv6Handler::get_layer_3(ethernet_packet.payload()),
        EtherTypes::Ipv4 => Ipv4Handler::get_layer_3(ethernet_packet.payload()),
        EtherTypes::Arp => ArpHandler::get_layer_3(ethernet_packet.payload()),
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
```

This function serves as the entry point for extracting Layer 3 information from Ethernet packets.

Certainly! I'll add a step to explain how to incorporate the `get_layer_3_infos(ethernet_packet)` function to retrieve Layer 3 information and update the `PacketInfos` struct with this data. Here's the additional step:

## Step 5: Retrieve Layer 3 Information

In this step, we'll retrieve Layer 3 information and update the `PacketInfos` struct with the obtained data. We'll make use of the `get_layer_3_infos` function from the `layer_3_infos` module, which we've previously defined.

```rust
use crate::info_packet::layer_3_infos::{get_layer_3_infos, Layer3Infos};
mod layer_3_infos;

pub struct PacketInfos {
    mac_address_source: MacAddr,
    mac_address_destination: MacAddr,
    interface: String,
    l_3_protocol: String,
    layer_3_infos: Layer3Infos
}

impl PacketInfos {
    pub fn new(interface_name: &String, ethernet_packet: &EthernetPacket<'_>) -> PacketInfos {
        PacketInfos {
            mac_address_source: ethernet_packet.get_source(),
            mac_address_destination: ethernet_packet.get_destination(),
            interface: interface_name.to_string(),
            l_3_protocol: ethernet_packet.get_ethertype().to_string(),
            layer_3_infos: get_layer_3_infos(ethernet_packet), // Retrieve Layer 3 information
        }
    }
}
```

In the `new` constructor method for the `PacketInfos` struct, we've added a line to call the `get_layer_3_infos` function with the `ethernet_packet` as its argument. This function returns Layer 3 information, and we store it in the `layer_3_infos` field of the `PacketInfos` struct.

By incorporating this step, your `PacketInfos` struct now includes Layer 3 information alongside Layer 2 details, making it a comprehensive data structure for representing and displaying packet information.

### Conclusion:

In this second part of our tutorial series, we've extended our packet handling application to include Layer 3 network packets. We've defined a `Layer3Infos` struct, implemented the `HandlePacket` trait, and created specific packet handlers for IPv4, IPv6, and ARP packets.

In the next part of the series, we'll continue enhancing