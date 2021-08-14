use pnet::datalink::NetworkInterface;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;

pub fn ethernet_frame_handler(interface: &NetworkInterface, ethernet: EthernetPacket) {
    let if_name = &interface.name;

    match ethernet.get_ethertype() {
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
