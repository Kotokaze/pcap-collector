pub mod header;
use header::PcapHeader;

pub mod msg;
use msg::PcapMsg;

use derive_builder::Builder;

/// Pcap file format struct
#[derive(Builder, Debug)]
pub struct Pcap {
    /// Pcap file header
    pub header: PcapHeader,

    /// Pcap messages
    pub body: Option<PcapMsg>,
}
