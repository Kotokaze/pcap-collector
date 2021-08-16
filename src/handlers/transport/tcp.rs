use pnet::packet::tcp::TcpPacket;
use pnet::packet::PacketSize;

use std::net::IpAddr;

pub fn tcp_handler(if_name: &str, src: IpAddr, dst: IpAddr, packet: TcpPacket) {
    println!(
        "[{:<}] TCP packet: {:>}:{:<} -> {:>}:{:<}; len: {:<}; seq: {:<}; ack: {:<}",
        if_name,
        src,
        packet.get_source(),
        dst,
        packet.get_destination(),
        packet.packet_size(),
        packet.get_sequence(),
        packet.get_acknowledgement(),
    )
}
