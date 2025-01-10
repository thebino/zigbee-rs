#[derive(Default)]
pub struct Config {
    pub radio_channel: u8,
    pub device_discovery_type: crate::zdo::config::DiscoveryType,
    /// This indicates the device class
    pub device_type: crate::apl::descriptors::LogicalType,
}

#[derive(Default)]
pub enum DiscoveryType {
    #[default]
    /// The IEEE address request is unicast to a particular device and assumes
    /// the NWK address is known.
    IEEE,
    /// The NWK address request is broadcast and carries the known IEEE address
    /// as data payload.
    NWK,
}
