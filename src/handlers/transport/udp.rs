use pnet::packet::udp::UdpPacket;
use std::net::IpAddr;

pub fn udp_handler(if_name: &str, src: IpAddr, dst: IpAddr, packet: UdpPacket) {
    println!(
        "[{:<}] UDP packet: {:>}:{:<} -> {:>}:{:<}; len: {:<}; checksum: {:<x}",
        if_name,
        src,
        packet.get_source(),
        dst,
        packet.get_destination(),
        packet.get_length(),
        packet.get_checksum(),
    )
}
