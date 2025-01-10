#![allow(dead_code)]

/// 3.2.2.3 NLME-NETWORK-DISCOVERY.request
pub struct NlmeNetworkDiscoveryRequest {
    pub(crate) scan_channels_list_structure: [u8; 8],
    pub(crate) scan_duration: u8,
}

/// 3.2.2.4 - NLME-NETWORK-DISCOVERY.confirm
pub struct NlmeNetworkDiscoveryConfirm {
    pub status: NlmeNetworkDiscoveryStatus,
    pub(crate) network_count: u8,
    pub(crate) network_descriptor: NetworkDescriptor,
}

#[derive(Debug, PartialEq)]
pub(crate) enum NlmeNetworkDiscoveryStatus {
    Successful,
}

pub struct NetworkDescriptor {
    pub extended_pan_id: u16,
    pub pan_id: u16,
    pub user_id: u8,
    pub logical_channel: u8,
    pub stack_profile: u8,
    pub zigbee_version: u8,
    pub beacon_order: u8,
    pub superframe_order: u8,
    pub permit_joining: bool,
    pub router_capacity: bool,
    pub end_device_capacity: bool,
}

/// 3.2.2.5 - NLME-NETWORK-FORMATION.request
pub struct NlmeNetworkFormationRequest {}
/// 3.2.2.6 - NLME-NETWORK-FORMATION.confirm
pub struct NlmeNetworkFormationConfirm {}

/// 3.2.2.7 - NLME-PERMIT-JOINING.request
pub struct NlmePermitJoiningRequest {}
/// 3.2.2.8 - NLME-PERMIT-JOINING.confirm
pub struct NlmePermitJoiningConfirm {}
/// 3.2.2.9 - NLME-START-ROUTER.request
pub struct NlmeStartRouterRequest {}
/// 3.2.2.10 - NLME-START-ROUTER.confirm
pub struct NlmeStartRouterConfirm {}
/// 3.2.2.11 - NLME-ED-SCAN.request
pub struct NlmeEdScanRequest {}
/// 3.2.2.12 - NLME-ED-SCAN.confirm
pub struct NlmeEdScanConfirm {}
/// 3.2.2.13 - NLME-JOIN.request
pub struct NlmeJoinRequest {
    pub(crate) extended_pan_id: u64,
    pub(crate) rejoin_network: u8,
    // ScanChannelsListStructure
    pub(crate) scan_duration: u8,
    // CapabilityInformation
    pub(crate) security_enabled: bool,
}
/// 3.2.2.14 - NLME-JOIN.indication
pub struct NlmeJoinIndication {
    pub(crate) network_address: u16,
    pub(crate) extended_address: u64,
    //CapabilityInformation
    pub(crate) rejoin_network: u8,
    pub(crate) secure_rejoin: bool,
}
/// 3.2.2.15 - NLME-JOIN.confirm
pub struct NlmeJoinConfirm {
    pub(crate) status: NlmeJoinStatus,
    pub(crate) network_address: u16,
    pub(crate) extended_pan_id: u64,
    // Channel List Structure
    pub(crate) enhanced_beacon_type: bool,
    pub(crate) mac_interface_index: u8,
}

pub(crate) enum NlmeJoinStatus {
    Success,
    InvalidRequest,
    NotPermitted,
    NoNetworks,
    // TODO: add more from 3.2.2.13.3
}

/// 3.2.2.16 - NLME-DIRECT-JOIN.request
pub struct NlmeDirectJoinRequest {}
/// 3.2.2.17 - NLME-DIRECT-JOIN.confirm
pub struct NlmeDirectJoinConfirm {}

/// 3.2.2.18 - NLME-LEAVE.request
pub struct NlmeLeaveRequest {}
/// 3.2.2.19 - NLME-LEAVE.indication
pub struct NlmeLeaveIndication {}
/// 3.2.2.20 - NLME-LEAVE.confirm
pub struct NlmeLeaveConfirm {}

/// 3.2.2.21 - NLME-RESET.request
pub struct NlmeResetRequest {}
/// 3.2.2.22 - NLME-RESET.confirm
pub struct NlmeResetConfirm {}
