#![allow(dead_code)]

use heapless::Vec;

use crate::common::types::MacCapability;

const CLUSTER_LIST_SIZE: usize = 2 * 0xffff;

/// 2.3.2.3 - Node Descriptor
pub(crate) struct NodeDescriptor {
    pub logical_type: LogicalType,
    pub complex_descriptor_available: bool,
    pub user_descriptor_available: bool,
    //aps_flags: unsupported for now
    pub frequency_band: FrequencyBand,
    pub mac_capability_flags: MacCapability,
    pub manufacturer_code: u16,
    pub maximum_buffer_size: u8,
    pub maximum_incoming_transfer_size: u16,
    pub server_mask: ServerMaskField,
    pub maximum_outgoing_transfer_size: u16,
    pub descriptor_capability_field: DescriptorCapabilityField,
}

/// 2.3.2.3.1 - Logical Type Field
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalType{
    Coordinator = 0b000,
    Router = 0b001,
    EndDevice = 0b010,
    // 011 - 111 reserved
}

/// 2.3.2.3.5 - Frequency Band Field
#[repr(u8)]
pub(crate) enum FrequencyBand{
    /// 868 - 868.6 MHz
    Low = 0,

    // reserved = 1

    /// 902 - 928 MHz
    Mid = 2,

    /// 2400 - 2483.5 MHz
    High = 3,
}

/// 2.3.2.3.10 - Server Mask Field
pub(crate) enum ServerMaskField {
    PrimaryTrustCenter = 0,
    BackupTrustCenter = 1,
    PrimaryBindingTableCache = 2,
    BackupBindingTableCache = 3,
    PrimaryDiscoveryCache = 4,
    BackupDiscoveryCache = 5,
    NetworkManager = 6,
    Reserved = 7,
    StackComplianceRevision = 8,
}

/// 2.3.2.3.12 - Descriptor Capability Field
pub enum DescriptorCapabilityField {
    ExtendedActiveEndpointListAvailable = 0,
    ExtendedSimpleDescriptorListAvailable = 1,
    // Reserved = 2–7 
}

/// 2.3.2.4 - Node Power Descriptor
struct NodePowerDescriptor {
    current_power_mode: u8,
    available_power_sources: u8,
    current_power_source: u8,
    current_power_source_level: u8,
}

/// 2.3.2.4.1 - Current Power Mode Field
pub enum CurrentPowerModeField {
    ReceiverSynchronized = 0b0000,
    Periodically = 0b0001,
    WhenStimulated = 0b0010,
    // Reserved = 0b0011 - 0b1111
}

/// 2.3.2.4.2 - Available Power Sources Field
/// 2.3.2.4.3 - Current Power Source Field
#[repr(u8)]
pub enum PowerSource {
    Constant = 0,
    RechargeableBattery = 1,
    DisposableBattery = 2,
    Reserved = 3,
}

/// 2.3.2.4.4 - Current Power Source Level Field
#[repr(u8)]
pub enum PowerLevel{
    Critical = 0b0000,
    /// 33%
    Low = 0b0100,
    /// 66%
    High = 0b1000,
    /// 100%
    Full = 0b1100,
}

/// 2.3.2.5 - Simple Descriptor
pub struct SimpleDescriptor {
    pub endpoint: u8,
    pub appl_prof_id: u16,
    pub appl_dev_id: u16,
    pub appl_dev_vers: u8,
    pub appl_input_clusters: Vec<u16, CLUSTER_LIST_SIZE>,
    pub appl_output_clusters: u8,
}

/// 2.3.2.7 - User Descriptor
pub struct UserDescriptor{
    pub descriptor: [u8; 16]
}

