# Title: Implementing Traits in Rust for Packet Handling with pnet - Part 1: Layer 2

## Introduction:

In this tutorial series, we'll explore how to create a modular and extensible packet handling application in Rust using traits. We'll start by implementing traits for handling Layer 2 network packets using the `pnet` library. Traits will help us define common behaviors for different packet types, enabling code reuse and flexibility.

## Prerequisites:

- Basic knowledge of Rust.
- Familiarity with the `pnet` library for packet manipulation.
- Prior knowledge of the previous tutorial in this series where we introduced the initial code for handling Layer 2 network packets. Understanding that code will provide context for the current tutorial, as we'll build upon it to leverage common behaviors among different Ethernet packet types.

Organizing Code into Modules:
To maintain a clean and organized codebase, it's essential to structure your Rust projects into modules. Modules help improve code readability, maintainability, and reusability by encapsulating related functionality into separate units.

Here's how you can organize your code into modules:
1. Create a New Module File:

    Start by creating a new Rust module file in your project's directory. You can name it something like info_packet.rs.

2. Define the Module in main.rs:

    In your main.rs file, declare the module using the mod keyword, specifying the name of the module file without the .rs extension. For example:

```rust
mod info_packet;
```

## Step 1: Create a Struct for Layer 2 Packet Information
Next, create a struct to represent Layer 2 packet information. This struct will hold details such as MAC addresses and the network interface name.

```rust
use pnet::packet::ethernet::EthernetPacket;
use pnet::util::MacAddr;

pub struct PacketInfos {
    mac_address_source: MacAddr,
    mac_address_destination: MacAddr,
    interface: String,
}
```

## Step 2: Implementing a Constructor for the `PacketInfos` Struct

In object-oriented programming (OOP), a constructor is a special method used to create and initialize objects of a class or struct. In Rust, we achieve this by implementing a method associated with the struct, typically named `new`. In our case, we want to create `PacketInfos` objects based on incoming Ethernet packets.

```rust
impl PacketInfos {
    // Constructor method to create a new PacketInfos object
    pub fn new(interface_name: &String, ethernet_packet: &EthernetPacket<'_>) -> PacketInfos {
        // Inside the constructor, we initialize the object's fields
        PacketInfos {
            // Field 1: MAC Address Source
            mac_address_source: ethernet_packet.get_source(),

            // Field 2: MAC Address Destination
            mac_address_destination: ethernet_packet.get_destination(),

            // Field 3: Network Interface Name
            interface: interface_name.to_string(),
        }
    }
}
```

Here's a breakdown of what's happening in the constructor:

1. **Method Definition:**
   - We define a method named `new` within the `impl PacketInfos` block. This method is associated with the `PacketInfos` struct and serves as a constructor.

2. **Parameters:**
   - The constructor takes two parameters:
     - `interface_name`: A reference to a `String`, representing the name of the network interface where the packet was received.
     - `ethernet_packet`: A reference to an `EthernetPacket`, which is the raw Ethernet packet data.

3. **Object Initialization:**
   - Inside the constructor, we initialize the fields of the `PacketInfos` object:
     - `mac_address_source`: We extract the source MAC address from the `ethernet_packet` using `ethernet_packet.get_source()`.
     - `mac_address_destination`: We similarly extract the destination MAC address.
     - `interface`: We store the `interface_name` as a `String`.

4. **Object Creation:**
   - Finally, the constructor returns a new `PacketInfos` object with the specified field values, creating an instance of the struct.

In summary, the constructor is responsible for creating a new `PacketInfos` object with the provided parameters, setting the MAC addresses and interface name based on the input Ethernet packet. This is a fundamental OOP concept, as constructors allow us to encapsulate object creation and initialization logic within a struct or class, promoting code reusability and maintainability.


## Step 3: Calling the Constructor in the main Function

Now that we have implemented the PacketInfos constructor, let's integrate it into the main function to create PacketInfos objects when processing Layer 2 packets. This step will allow us to extract and display relevant information from these packets.

```rust
fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
    /** */
    };

    println!("Start thread reading packet on interface: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    let packet_info = info_packet::PacketInfos::new(&interface.name, &ethernet_packet);
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
```

## Step 4: Display Layer 2 Packet Information 
Implement the `Display` trait for the `PacketInfos` struct to format and display Layer 2 packet information.

```rust
use std::fmt;
impl fmt::Display for PacketInfos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the formatting for PacketInfos here
        write!(f, "MAC Source: {}\n", self.mac_address_source)?;
        write!(f, "MAC Destination: {}\n", self.mac_address_destination)?;
        write!(f, "Interface: {}\n", self.interface)?;
        // Format other fields as needed

        Ok(())
    }
}
```

## Step 5: Display Packet Infos

In this step, we'll integrate the PacketInfos display functionality into your packet processing code. This allows us to visualize packet details as they are processed.
```rust
    println!("Start thread reading packet on interface: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    let packet_info = info_packet::PacketInfos::new(&interface.name, &ethernet_packet);
                    println!("{}", packet_info);
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
```
In this code:

- We use the println! macro to indicate the start of reading a packet on the specified network interface.

- Inside the loop, when a packet is received and validated, we create a PacketInfos object as before.

- The key addition is the use of println!("{}", packet_info) to display the packet information. Thanks to the Display trait implementation, this line formats and prints the PacketInfos object in a human-readable way.

By following these steps, you can efficiently display Layer 2 packet information as part of your network packet processing code. This makes it easier to observe and understand the packet details as they are processed in real-time.

## Conclusion:

In this tutorial, we've learned how to implement traits in Rust to handle Layer 2 network packets using the pnet library. We created a PacketInfos struct to represent Layer 2 packet information and implemented a constructor method for it. We also used the Display trait to format and display packet information.

By organizing our code into modules and following object-oriented programming principles, we've created a foundation for handling network packets efficiently and with code maintainability in mind.

In the next part of this series, we'll continue to build on this foundation by extending our packet handling application to include Layer 3 information handling. Stay tuned for Part 2, where we'll delve into handling Layer 3 packets and further enhancing our network packet processing capabilities!
