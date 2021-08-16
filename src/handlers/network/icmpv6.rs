use pnet::packet::icmpv6::Icmpv6Packet;
use std::net::IpAddr;

/// ICMP v6 packet handler
pub fn icmpv6_handler(if_name: &str, src: IpAddr, dst: IpAddr, packet: Icmpv6Packet) {
    println!(
        "[{0:<}] ICMPv6 Packet: {1:^} -> {2:^} (type: {3:?})",
        if_name,
        src,
        dst,
        packet.get_icmpv6_type(),
    )
}
