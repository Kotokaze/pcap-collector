mod icmp;
use icmp::icmp_handler;

mod icmpv6;
use icmpv6::icmpv6_handler;

use super::transport::transport_layer_handler;

use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::{icmp::IcmpPacket, icmpv6::Icmpv6Packet};

use std::net::IpAddr;

/// Network layer protocol handler
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

        IpNextHeaderProtocols::Icmpv6 => {
            let packet: Option<Icmpv6Packet> = Icmpv6Packet::new(payload);

            if let Some(packet) = packet {
                icmpv6_handler(if_name, src, dst, packet);
            } else {
                println!("[{:<}] ICMPv6 Packet:\tMalformed packet", if_name);
            }
        }

        _ => {
            transport_layer_handler(if_name, src, dst, proto, payload);
        }
    }
}
