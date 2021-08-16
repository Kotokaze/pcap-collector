mod icmp;
use icmp::icmp_handler;

use super::transport::transport_layer_handler;

use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};

use std::net::IpAddr;

pub fn network_layer_handler(
    if_name: &str,
    src: IpAddr,
    dst: IpAddr,
    proto: IpNextHeaderProtocol,
    payload: &[u8],
) {
    match proto {
        IpNextHeaderProtocols::Icmp => {
            let packet: Option<IcmpPacket> = IcmpPacket::new(payload);

            if let Some(packet) = packet {
                icmp_handler(if_name, src, dst, packet, payload);
            } else {
                println!("[{:<}] ICMP Packet:\tMalformed packet", if_name);
            }
        }

        _ => {
            transport_layer_handler(if_name, src, dst, proto, payload);
        }
    }
}
