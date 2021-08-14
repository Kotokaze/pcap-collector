use pnet::datalink;
use pnet::datalink::{Channel::Ethernet, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;

use pcap_collector::handlers::ethernet_frame_handler;

const IF_NAME: &str = "enp0s8";

fn main() {
    // Specify the interface to use
    let iface_name_match = |iface: &NetworkInterface| {
        return (iface.name == String::from(IF_NAME)) && iface.is_up();
    };

    // Find network interface
    let interfaces: Vec<NetworkInterface> = datalink::interfaces();
    let interface: NetworkInterface = interfaces.into_iter().find(iface_name_match).unwrap();

    // Create a channel to deal with the L2 packets
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("[ERROR] Unhandled channel type"),
        Err(e) => panic!("[ERROR] Unabel to create the datalink channel: {}", e),
    };

    println!(
        "Listening on {} at {}",
        &interface.ips.first().unwrap(),
        &interface.name
    );

    loop {
        match rx.next() {
            Ok(pkt) => {
                let ethernet = EthernetPacket::new(pkt).unwrap();
                ethernet_frame_handler(&interface, ethernet);
            }
            Err(e) => {
                panic!("[ERROR] Unable to read packet: {}", e);
            }
        }
    }
}
