#![allow(dead_code)]

use serde_derive::{Serialize, Deserialize};

mod types;

trait ApsdeSap {

}

type DstAddrMode = u8;
// 2.2.4.1.1
struct ApsdeSapRequest {
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

#[derive(Serialize, Deserialize)]
enum ApsdeSapConfirmStatus {
    Success, 
    NoShortAddress, 
    NoBoundDevice, 
    SecurityFail,
    NoAck, 
    AsduTooLong
}
// 2.2.4.1.2
struct ApsdeSapConfirm {
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
    src_endpoint: types::SrcEndpoint,
    status: ApsdeSapConfirmStatus,
    tx_time: u8,
}

#[derive(Serialize, Deserialize)]
enum ApsdeSapIndicationStatus {
    Success, 
    DefragUnsupported, 
    DefragDeferred
}
#[derive(Serialize, Deserialize)]
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

