mod tcp;
use tcp::tcp_handler;

mod udp;
use udp::udp_handler;

use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::{tcp::TcpPacket, udp::UdpPacket};

use std::net::IpAddr;

/// Transport layer packet handler
pub fn transport_layer_handler(
    if_name: &str,
    src: IpAddr,
    dst: IpAddr,
    proto: IpNextHeaderProtocol,
    payload: &[u8],
) {
    match proto {
        IpNextHeaderProtocols::Udp => {
            let packet: Option<UdpPacket> = UdpPacket::new(payload);

            if let Some(packet) = packet {
                udp_handler(if_name, src, dst, packet);
            } else {
                println!("[{:<}] UDP packet: Malformed packet", if_name);
            }
        }

        IpNextHeaderProtocols::Tcp => {
            let packet: Option<TcpPacket> = TcpPacket::new(payload);

            if let Some(packet) = packet {
                tcp_handler(if_name, src, dst, packet);
            } else {
                println!("[{:<}] TCP packet: Malformed packet", if_name);
            }
        }

        _ => {
            println!(
                "[{:<}] Unkown {:^} packet: {:^} -> {:^}; proto: {:<}; len: {:<x}",
                if_name,
                match src {
                    IpAddr::V4(..) => "IPv4",
                    IpAddr::V6(..) => "IPv6",
                },
                src,
                dst,
                proto,
                payload.len(),
            )
        }
    }
}
