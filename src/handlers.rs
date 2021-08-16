mod arp;
use arp::arp_handler;

mod network;
use network::network_layer_handler;

mod transport;

use pnet::datalink::NetworkInterface;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;

use std::net::IpAddr;

/// Extract ethernet frame
pub fn ethernet_frame_handler(interface: &NetworkInterface, ethernet: EthernetPacket) {
    let if_name = &interface.name;

    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let packet: Option<Ipv4Packet> = Ipv4Packet::new(ethernet.payload());

            if let Some(packet) = packet {
                let src = &packet.get_source();
                let dst = &packet.get_destination();
                let next_proto = &packet.get_next_level_protocol();
                let payload = &packet.payload();

                network_layer_handler(
                    if_name,
                    IpAddr::V4(*src),
                    IpAddr::V4(*dst),
                    *next_proto,
                    *payload,
                );
            } else {
                println!("[{:<}] IPv4 packet:\tMalformed packet", if_name);
            }
        }

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
