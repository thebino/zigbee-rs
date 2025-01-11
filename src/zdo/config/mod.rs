/// zigbee configuration
#[derive(Default)]
pub struct Config {
    /// The radio channel to operate on
    pub radio_channel: u8,
    /// Discovery type
    pub device_discovery_type: DiscoveryType,
    /// This indicates the device class
    pub device_type: crate::apl::descriptors::LogicalType,
}

/// Discovery Type
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
