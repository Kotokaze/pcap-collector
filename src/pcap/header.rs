use derive_builder::Builder;

/// Pcap file header struct
#[derive(Builder, Debug, Clone)]
pub struct PcapHeader {
    /// magic number
    pub magic_number: u32,

    /// major version number
    pub version_major: u16,

    /// minor version number
    pub version_minor: u16,

    /// GMT to local correction
    pub thiszone: i32,

    /// accuracy of timestamps
    pub sigfigs: u32,

    /// max length of captured packets, in octets
    pub snaplen: u32,

    /// data link type
    pub network: u32,
}
