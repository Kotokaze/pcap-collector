use derive_builder::Builder;

/// Pcap record header struct
#[derive(Builder, Debug, Clone)]
pub struct PcapRecHeader {
    /// timestamp in seconds (since January 1, 1970 00:00:00 GMT)
    pub ts_sec: u32,

    /// timestamp in microseconds
    pub ts_usec: u32,

    /// number of octets of packet saved in file
    pub incl_len: u32,

    /// actual length of packet
    pub orig_len: u32,
}
