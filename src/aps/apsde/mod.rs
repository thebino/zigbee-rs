//! Application Support Sub-Layer Data Entity
//!
//! The APSDE shall provide a data service to the network layer and both ZDO and application
//! objects to enable the transport of application PDUs between two or more devices.
//!
//! it will provide:
//! * Generation of the application level PDU (APDU)
//! * Binding
//! * Group address filtering
//! * Reliable transport
//! * Duplicate rejection
//! * Fragmentation
//!
#![allow(dead_code)]
use crate::aps::types;

use super::types::{Address, DstAddrMode, SrcAddrMode, TxOptions};

/// Application support sub-layer data entity – service access point
///
/// 2.2.4.1.1
///
/// Interface between the NWK (Network) layer and the APL (Application) layer
/// through a general set of services for use by both the ZDO (device object) and the application.
pub trait ApsdeSap {
    /// Requests the transfer of a NHLE PDU from a local NHLE to one or more peer NHLE entities
    fn data_request(&self, request: ApsdeSapRequest) -> ApsdeSapConfirm;
}

// 2.2.4.1.1
pub struct ApsdeSapRequest {
    dst_addr_mode: DstAddrMode,
    dst_address: Address,
    dst_endpoint: u8,
    profile_id: u16,
    cluster_id: u16,
    src_endpoint: types::SrcEndpoint,
    asdulength: u8,
    asdu: u8,
    tx_options: TxOptions,
    use_alias: bool,
    alias_src_addr: u16,
    alias_seq_number: u8,
    radius_counter: u8,
}

/// The status of the corresponding request.
pub enum ApsdeSapConfirmStatus {
    /// indicating that the request to transmit was successful
    Success,
    /// No corresponding 16-bit NKW address found
    NoShortAddress,
    /// No binding table entries found with the respectively SrcEndpoint and ClusterId parameter
    NoBoundDevice,
    /// the security processing failed
    SecurityFail,
    /// one or more APS acknowledgements were not correctly received
    NoAck,
    /// ASDU to be transmitted is larger than will fit in a single frame and fragmentation is not possible
    AsduTooLong,
}
// 2.2.4.1.2
pub struct ApsdeSapConfirm {
    pub dst_addr_mode: DstAddrMode,
    pub dst_address: u8,
    pub dst_endpoint: u8,
    pub src_endpoint: types::SrcEndpoint,
    pub status: ApsdeSapConfirmStatus,
    pub tx_time: u8,
}

pub enum ApsdeSapIndicationStatus {
    Success,
    DefragUnsupported,
    DefragDeferred,
}

pub enum SecurityStatus {
    Unsecured,
    SecuredNwkKey,
    SecuredLinkKey,
}

// 2.2.4.1.3
pub struct ApsdeSapIndication {
    dst_addr_mode: DstAddrMode,
    dst_address: u8,
    dst_endpoint: u8,
    src_addr_mode: SrcAddrMode,
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
