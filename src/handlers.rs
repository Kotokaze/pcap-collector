mod arp;
use arp::arp_handler;

use pnet::datalink::NetworkInterface;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;

pub fn ethernet_frame_handler(interface: &NetworkInterface, ethernet: EthernetPacket) {
    let if_name = &interface.name;

    match ethernet.get_ethertype() {
        EtherTypes::Arp => {
            let packet: Option<ArpPacket> = ArpPacket::new(ethernet.payload());

            if let Some(packet) = packet {
                arp_handler(if_name, packet);
            } else {
                println!("[{}] ARP packet:\tMalformed packet", if_name);
            }
        }

        _ => {
            println!(
                "[{}] Unknown packet: {} -> {}; etherType: {:?} length: {}",
                if_name,
                ethernet.get_source(),
                ethernet.get_destination(),
                ethernet.get_ethertype(),
                ethernet.packet().len(),
            )
        }
    };
}
