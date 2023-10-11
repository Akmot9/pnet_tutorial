use pnet::datalink::Channel::Ethernet;
use pnet::datalink;
use pnet::packet::ethernet::EthernetPacket;

fn main() {

    let interfaces = datalink::interfaces();
    // Choose an interface
    let interface = interfaces[5].clone();
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}",&interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start reading packet: ");
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("New packet:");
                    println!("{} => {}: {}",ethernet_packet.get_destination(),ethernet_packet.get_source(),ethernet_packet.get_ethertype());
                }
            }
            Err(e)=> {
                panic!("An error occurred while reading: {}", e);
            }
        }
        }


}
