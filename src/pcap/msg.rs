pub mod rec_header;
use derive_builder::Builder;
use rec_header::PcapRecHeader;

/// Pcap file payload struct
#[derive(Builder, Debug, Clone)]
pub struct PcapMsg {
    /// Pcap record header
    pub header: PcapRecHeader,

    // /// Pcap packet data
    // data: ,
}
