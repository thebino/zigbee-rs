#![allow(dead_code)]
use crate::aps::types;

type DstAddrMode = u8;
// 2.2.4.3.1 APSME-BIND.request
struct ApsmeBindRequest {
    src_address: u64,
    src_endpoint: types::SrcEndpoint,
    cluster_id: u16,
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
}

enum ApsmeBindRequestStatus {
    Success,
    IllegalRequest,
    TableFull,
    NotSupported
}

// 2.2.4.3.2 APSME-BIND.confirm
struct ApsmeBindConfirm {
    status: ApsmeBindRequestStatus,
    src_address: u64,
    src_endpoint: types::SrcEndpoint,
    cluster_id: u16,
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
}

// 2.2.4.3.3 APSME-UNBIND.request
struct ApsmeUnbindRequest {
    src_address: u64,
    src_endpoint: types::SrcEndpoint,
    cluster_id: u16,
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
}

enum ApsmeUnbindRequestStatus {
    Success,
    IllegalRequest,
    InvalidBinding
}
// 2.2.4.3.4 APSME-UNBIND.confirm
struct ApsmeUnbindConfirm {
    status: ApsmeUnbindRequestStatus,
    src_address: u64,
    src_endpoint: types::SrcEndpoint,
    cluster_id: u16,
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
}

enum AIBAttribute {
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

// 2.2.4.4.1 APSME-GET.request
struct ApsmeGetRequest {
    attribute: AIBAttribute
}

struct AIBAttributeValue {

}

struct ApsmeGetConfirm {
    status: u8,
    attribute: AIBAttribute,
    attribute_length: u8,
    attribute_value: AIBAttributeValue,
}

