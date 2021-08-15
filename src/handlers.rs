use pnet::datalink::NetworkInterface;
use pnet::packet::arp::{ArpOperation, ArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;

pub fn ethernet_frame_handler(interface: &NetworkInterface, ethernet: EthernetPacket) {
    let if_name = &interface.name;

    match ethernet.get_ethertype() {
        EtherTypes::Arp => {
            let packet: Option<ArpPacket> = ArpPacket::new(ethernet.payload());

            if let Some(packet) = packet {
                println!(
                    "[{}] ARP packet: {}({}) -> {}({}); operation: {}",
                    if_name,
                    packet.get_sender_proto_addr(),
                    packet.get_sender_hw_addr(),
                    packet.get_target_proto_addr(),
                    packet.get_target_hw_addr(),
                    match packet.get_operation() {
                        ArpOperation(1) => "request",
                        ArpOperation(2) => "reply",
                        _ => "unknown",
                    },
                )
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
