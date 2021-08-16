use pnet::packet::arp::{ArpOperation, ArpPacket};

/// Arp packet handler
pub fn arp_handler(if_name: &str, arp_packet: ArpPacket) {
    println!(
        "[{:<}] ARP packet: {:>}({:<}) -> {:>}({:<}); operation: {:^}",
        if_name,
        arp_packet.get_sender_proto_addr(),
        arp_packet.get_sender_hw_addr(),
        arp_packet.get_target_proto_addr(),
        arp_packet.get_target_hw_addr(),
        match arp_packet.get_operation() {
            ArpOperation(1) => "request",
            ArpOperation(2) => "reply",
            _ => "unknown",
        },
    )
}
