#![allow(dead_code)]
use crate::aps::types;

// 2.2.4.1
// Application support sub-layer data entity – service access point
//
// Interface between the NWK (Network) layer and the APL (Application) layer
// through a general set of services for use by both the ZDO (device object) and the application.
pub(crate) trait ApsdeSap {
    fn data_request(request: ApsdeSapRequest) -> ApsdeSapConfirm;
}



type DstAddrMode = u8;
// 2.2.4.1.1
pub(crate) struct ApsdeSapRequest {
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
    profile_id: u16,
    cluster_id: u16,
    src_endpoint: types::SrcEndpoint,
    asdulength: u8,
    asdu: u8,
    tx_options: bitmaps::Bitmap<8>,
    use_alias: bool,
    alias_src_addr: u16,
    alias_seq_number: u8,
    radius_counter: u8
}

enum ApsdeSapConfirmStatus {
    Success, 
    NoShortAddress, 
    NoBoundDevice, 
    SecurityFail,
    NoAck, 
    AsduTooLong
}
// 2.2.4.1.2
pub(crate) struct ApsdeSapConfirm {
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
    src_endpoint: types::SrcEndpoint,
    status: ApsdeSapConfirmStatus,
    tx_time: u8,
}

enum ApsdeSapIndicationStatus {
    Success, 
    DefragUnsupported, 
    DefragDeferred
}
enum SecurityStatus {
    Unsecured, 
    SecuredNwkKey, 
    SecuredLinkKey
}

// 2.2.4.1.3
struct ApsdeSapIndication {
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
    src_addr_mode: u8,
    src_address: u64,
    src_endpoint: types::SrcEndpoint,
    profile_id: u16,
    cluster_id: u16,
    asdulength: u8,
    status: ApsdeSapIndicationStatus,
    security_status: SecurityStatus,
    link_quality: u8,
    rx_time: u8,
}

