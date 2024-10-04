#![allow(dead_code)]
//!
//! 2.2.4.4 Information Base Maintenance
//! This set of primitives defines how the next higher layer of a device can read and write attributes in the AIB
//!
use crate::aps::types::{self, Address};

type DstAddrMode = u8;
/// 2.2.4.3.1 - APSME-BIND.request
pub struct ApsmeBindRequest {
    pub src_address: Address,
    pub src_endpoint: types::SrcEndpoint,
    pub cluster_id: u16,
    pub dst_addr_mode: DstAddrMode,
    pub dst_address: u8,
    pub dst_endpoint: u8,
}

#[derive(Debug, PartialEq)]
pub enum ApsmeBindRequestStatus {
    Success,
    IllegalRequest,
    TableFull,
    NotSupported,
}

/// 2.2.4.3.2 - APSME-BIND.confirm
pub struct ApsmeBindConfirm {
    pub(crate) status: ApsmeBindRequestStatus,
    pub src_address: Address,
    pub src_endpoint: types::SrcEndpoint,
    pub cluster_id: u16,
    pub dst_addr_mode: DstAddrMode,
    pub dst_address: u8,
    pub dst_endpoint: u8,
}

/// 2.2.4.3.3 - APSME-UNBIND.request
pub struct ApsmeUnbindRequest {
    pub(crate) src_address: Address,
    pub(crate) src_endpoint: types::SrcEndpoint,
    pub(crate) cluster_id: u16,
    pub(crate) dst_addr_mode: DstAddrMode,
    pub(crate) dst_address: u8,
    pub(crate) dst_endpoint: u8,
}

pub enum ApsmeUnbindRequestStatus {
    Success,
    IllegalRequest,
    InvalidBinding,
}
/// 2.2.4.3.4 - APSME-UNBIND.confirm
pub struct ApsmeUnbindConfirm {
    pub(crate) status: ApsmeUnbindRequestStatus,
    pub(crate) src_address: Address,
    pub(crate) src_endpoint: types::SrcEndpoint,
    pub(crate) cluster_id: u16,
    pub(crate) dst_addr_mode: DstAddrMode,
    pub(crate) dst_address: u8,
    pub(crate) dst_endpoint: u8,
}

#[derive(Debug, PartialEq)]
pub enum AIBAttribute {
    IapsBindingTable = 0xc1,
    ApsDesignatedCoordinator = 0xc2,
    ApsChannelMaskList = 0xc3,
    ApsUseExtendedPANID = 0xc4,
    ApsGroupTable = 0xc5,
    ApsNonmemberRadius = 0xc6,
    ApsUseInsecureJoin = 0xc8,
    ApsInterframeDelay = 0xc9,
    ApsLastChannelEnergy = 0xca,
    ApsLastChannelFailureRate = 0xcb,
    ApsChannelTimer = 0xcc,
    ApsMaxWindowSize = 0xcd,
    ApsParentAnnounceTimer = 0xce,
}

/// 2.2.4.4.1 - APSME-GET.request
pub struct ApsmeGetRequest {
    attribute: AIBAttribute,
}

pub struct AIBAttributeValue {}
/// 2.2.4.4.2 - APSME-GET.confirm
pub struct ApsmeGetConfirm {
    pub(crate) status: ApsmeGetConfirmStatus,
    pub(crate) attribute: AIBAttribute,
    pub(crate) attribute_length: u8,
    pub(crate) attribute_value: AIBAttributeValue,
}

#[derive(Debug, PartialEq)]
pub enum ApsmeGetConfirmStatus {
    Success,
    UnsupportedAttribute
}

/// 2.2.4.4.3 - APSME-SET.request
pub struct ApsmeSetRequest {
    attribute: AIBAttribute,
    attribute_length: u8,
    attribute_value: AIBAttributeValue,
}

/// 2.2.4.4.4 - APSME-SET.confirm
pub struct ApsmeSetConfirm {
    status: u8,
    attribute: AIBAttribute,
    attribute_length: u8,
    attribute_value: AIBAttributeValue,
}

/// 2.2.4.5.1 - APSME-ADD-GROUP.request
pub struct ApsmeAddGroupRequest {}

/// 2.2.4.5.2 - APSME-ADD-GROUP.confirm
pub struct ApsmeAddGroupConfirm {}

/// 2.2.4.5.3 - APSME-REMOVE-GROUP.request
pub struct ApsmeRemoveGroupRequest {}
/// 2.2.4.5.3 - APSME-REMOVE-GROUP.request
pub struct ApsmeRemoveGroupConfirm {}

/// 2.2.4.5.5 - APSME-REMOVE-ALL-GROUPS.request
pub struct ApsmeRemoveAllGroupsRequest {}
/// 2.2.4.5.6 - APSME-REMOVE-ALL-GROUPs.request
pub struct ApsmeRemoveAllGroupsConfirm {}
