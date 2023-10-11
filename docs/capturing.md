# Tutorial: Capturing Network Packets with `pnet` in Rust

## Introduction
- This tutorial will guide you through the process of capturing network packets in real-time using the `pnet` library in Rust.
- Packet capturing is an essential task in network monitoring, analysis, and debugging.

## Setting up the Environment
- Make sure you have an existing Rust project with `pnet` added as a dependency in your `Cargo.toml`:
```toml
[dependencies]
pnet = "0.34.0"
```

## Choosing a Network Interface
In this example, we will select the network interface with index 5 from the list of available interfaces. Adjust the index according to your specific case.

```rust
let interface = interfaces[5].clone();
```

## Creating a Datalink Channel
We will create a datalink channel for the chosen interface. This channel will allow us to read incoming packets.

```rust
let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
    Ok(Ethernet(tx, rx)) => (tx, rx),
    Ok(_) => panic!("Unhandled channel type: {}", &interface),
    Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
};
```

## Capturing Packets
We will set up a loop to continuously read and handle incoming packets.

```rust
use pnet::packet::ethernet::EthernetPacket;

println!("Start reading packets: ");
loop {
    match rx.next() {
        Ok(packet) => {
            if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                println!("New packet:");
                println!("{:?}", ethernet_packet);
            }
        },
        Err(e) => {
            panic!("An error occurred while reading: {}", e);
        }
    }
}
```
- In this code snippet, we are using the `EthernetPacket::new()` method to parse the raw packet data into a more manageable `EthernetPacket` object. 
- We then print the details of the Ethernet packet to the console.

## Full Code Example
Here's how your full main function might look:

```rust
extern crate pnet;

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;

fn main() {
    let interfaces = datalink::interfaces();
    let interface = interfaces[5].clone(); // Choose your own interface index

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e),
    };

    println!("Start reading packets: ");
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("New packet:");
                    println!("{:?}", ethernet_packet);
                }
            },
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
```

## Running with Superuser Permissions

### Why Superuser Permissions are Needed
Packet capturing typically requires superuser (root) permissions to access low-level network interfaces. Failing to run the program with the necessary permissions will result in an "Operation not permitted" error.

### How to Run Your Program with Superuser Permissions

#### Build the Program First
Before running the program with superuser permissions, build it using Cargo. This ensures you have an executable binary to run.
```bash
cargo build
```

#### Execute the Binary with Superuser Permissions
After building, run the compiled binary with `sudo` to grant it the necessary permissions.
```bash
sudo ./target/debug/pnet_tutorial
```
This is the result you should have: 
```bash
Start reading packet: 
New packet:
EthernetPacket { destination : 0a:87:c7:e6:6f:64, source : 0a:87:c7:6e:e9:f1, ethertype : EtherType(34525),  }
New packet:
EthernetPacket { destination : 0a:87:c7:e6:6f:64, source : 0a:87:c7:6e:e9:f1, ethertype : EtherType(2048),  }

```

### Important Note on Security
Running any program with superuser privileges can be risky if the code (or the libraries it uses) has not been thoroughly reviewed and trusted. Be sure you understand the code you are running as root and its implications.


## Conclusion
By the end of this tutorial, you should be able to capture network packets in real-time using Rust and the `pnet` library. This opens up a variety of possibilities in network analysis, monitoring, and debugging.
