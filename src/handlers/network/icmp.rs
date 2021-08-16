use pnet::packet::icmp::{echo_reply::EchoReplyPacket, echo_request::EchoRequestPacket};
use pnet::packet::icmp::{IcmpPacket, IcmpTypes};
use std::net::IpAddr;

pub fn icmp_handler(if_name: &str, src: IpAddr, dst: IpAddr, packet: IcmpPacket, payload: &[u8]) {
    match packet.get_icmp_type() {
        IcmpTypes::EchoRequest => {
            let echo_request_packet: Option<EchoRequestPacket> = EchoRequestPacket::new(payload);

            if let Some(echo_request_packet) = echo_request_packet {
                println!(
                    "[{:<}] Echo (ping) request: {:^} -> {:^}; (seq: {:<}; id: {:<})",
                    &if_name,
                    &src,
                    &dst,
                    &echo_request_packet.get_sequence_number(),
                    &echo_request_packet.get_identifier(),
                );
            } else {
                println!("[{:<}] Echo (ping) request:\tMalformed packet", if_name);
            }
        }

        IcmpTypes::EchoReply => {
            let echo_reply_packet: Option<EchoReplyPacket> = EchoReplyPacket::new(payload);

            if let Some(echo_reply_packet) = echo_reply_packet {
                println!(
                    "[{:<}] Echo (ping) reply: {:^} -> {:^}; (seq: {:<}; id: {:<})",
                    &if_name,
                    &src,
                    &dst,
                    &echo_reply_packet.get_sequence_number(),
                    &echo_reply_packet.get_identifier(),
                );
            } else {
                println!("[{:<}] Echo (ping) reply:\tMalformed packet", if_name);
            }
        }

        _ => {
            println!(
                "[{:<}] ICMP Packet: {:^} -> {:^}; type: {:?}; code: {:?}; checksum: {:<}",
                if_name,
                src,
                dst,
                packet.get_icmp_type(),
                packet.get_icmp_code(),
                packet.get_checksum(),
            );
        }
    }
}
