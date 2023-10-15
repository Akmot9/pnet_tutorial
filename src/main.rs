use pnet::datalink::Channel::Ethernet;
use pnet::datalink;
use pnet::packet::ethernet::EthernetPacket;
use pnet::datalink::NetworkInterface;

use std::thread;

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

fn capture_packets(interface: NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, 
                                                                    Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}",&interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start reading packet: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    info_packet::process_packet_by_type(&interface.name, &ethernet_packet)
                }
            }
            Err(e)=> {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}


