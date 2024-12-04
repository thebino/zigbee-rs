//! Application Support Sub-Layer Management Entity
//!
//! The APSME shall provide a management service to allow an application to interact with the stack
//!
//! it provices the following services:
//! * Binding management
//! * AIB management
//! * Security
//! * Group management
//!
#![allow(dead_code)]

use basemgt::{AIBAttribute, AIBAttributeValue, ApsmeBindConfirm, ApsmeBindRequest, ApsmeBindRequestStatus, ApsmeGetConfirm, ApsmeGetConfirmStatus, ApsmeUnbindConfirm, ApsmeUnbindRequest, ApsmeUnbindRequestStatus};
use heapless::Vec;
use super::types::Address;

pub mod basemgt;
pub mod groupmgt;

/// Application support sub-layer management service - service access point
///
/// 2.2.4.2
///
/// supports the transport of management commands between the NHLE and the APSME
pub trait ApsmeSap {
    /// 2.2.4.3.1 - request to bind two devices together, or to bind a device to a group
    fn bind_request(&mut self, request: ApsmeBindRequest) -> ApsmeBindConfirm;
    /// 2.2.4.3.3 - request to unbind two devices, or to unbind a device from a group
    fn unbind_request(&mut self, request: ApsmeUnbindRequest) -> ApsmeUnbindConfirm ;
    /// 2.2.4.4.1
    fn get(&self, attribute: AIBAttribute) -> ApsmeGetConfirm;
    /// 2.2.4.4.3
    fn set();
    /// 2.2.4.5.1
    fn add_group();
    /// 2.2.4.5.1
    fn remove_group();
    /// 2.2.4.5.5
    fn remove_all_groups();
}

struct Apsme {
    pub(crate) supports_binding_table: bool,
    // TODO: limit the size
    pub(crate) binding_table: Vec<Address, 265>,
    pub(crate) joined_network: Option<Address>,
    pub(crate) database: Vec<AIBAttribute, 265>,
}

impl  Apsme {
    fn new() -> Apsme {
        Self {
            supports_binding_table: true,
            binding_table: Vec::new(),
            joined_network: None,
            database: Vec::new(),
        }
    }
    fn is_joined(&self) -> bool {
        self.joined_network.is_some()
    }
    fn is_full(&self) -> bool {
        self.binding_table.len() >= self.binding_table.capacity()
    }
    fn add_entry(&mut self, address: Address) -> Result<(), Address> {
        self.binding_table.push(address)
    }
}

impl ApsmeSap for Apsme {
    fn bind_request(&mut self, request: ApsmeBindRequest) -> ApsmeBindConfirm {
        let status = if !self.is_joined() || !self.supports_binding_table {
            ApsmeBindRequestStatus::IllegalRequest
        } else if self.is_full() {
            ApsmeBindRequestStatus::TableFull
        } else {
            self.add_entry(request.src_address.clone()).expect("Could not add entry in binding table");
            ApsmeBindRequestStatus::Success
        };

        ApsmeBindConfirm {
            status,
            src_address: request.src_address,
            src_endpoint: request.src_endpoint,
            cluster_id: request.cluster_id,
            dst_addr_mode: request.dst_addr_mode,
            dst_address: request.dst_address,
            dst_endpoint: request.dst_endpoint,
        }

    }

    fn unbind_request(&mut self, request: ApsmeUnbindRequest) -> ApsmeUnbindConfirm {
        let status = if !self.is_joined() || !self.supports_binding_table {
            ApsmeUnbindRequestStatus::IllegalRequest
        } else if !self.binding_table.contains(&request.src_address) {
            ApsmeUnbindRequestStatus::InvalidBinding
        } else {
            ApsmeUnbindRequestStatus::Success

        };

        ApsmeUnbindConfirm {
            status,
            src_address: request.src_address,
            src_endpoint: request.src_endpoint,
            cluster_id: request.cluster_id,
            dst_addr_mode: request.dst_addr_mode,
            dst_address: request.dst_address,
            dst_endpoint: request.dst_endpoint,
        }
    }

    fn get(&self, attribute: AIBAttribute) -> ApsmeGetConfirm {
        ApsmeGetConfirm {
            status: ApsmeGetConfirmStatus::UnsupportedAttribute,
            attribute,
            attribute_length: 0u8,
            attribute_value: AIBAttributeValue {},
        }
        // let attr = self.database.get(attribute);
        // return if attr.is_none() {
            // return ApsmeGetConfirm {
            //     status: ApsmeGetConfirmStatus::UnsupportedAttribute,
            //     attribute: attribute,
            //     attribute_length: attr.len(),
            //     attribute_value: todo!(),
            // }
        // } else {
        //     ApsmeGetConfirm {
        //         status: ApsmeGetConfirmStatus::Success,
        //         attribute: attr.unwrap(),
        //         attribute_length: attr.len(),
        //         attribute_value: todo!(),
        //     }
        // }
    }

    fn set() {
        todo!()
    }

    fn add_group() {
        todo!()
    }

    fn remove_group() {
        todo!()
    }

    fn remove_all_groups() {
        todo!()
    }
}

// TODO: add AIB (APS information base) a database of managed objects
//
// 2.2.4.4.1

#[cfg(test)]
mod tests {
    use basemgt::ApsmeBindRequestStatus;

    use crate::aps::types::SrcEndpoint;

    use super::*;

    // 2.2.4.3.1
    #[test]
    fn bind_request_device_does_not_support_binding_should_fail() {
        // given
        let mut apsme = Apsme::new();
        apsme.supports_binding_table = false;
        let request = ApsmeBindRequest {
            src_address: Address::Extended(0u64),
            src_endpoint: SrcEndpoint::new(10).unwrap_or(SrcEndpoint { value: 0 }),
            cluster_id: 1u16,
            dst_addr_mode: 0u8,
            dst_address: 1u8,
            dst_endpoint: 2u8,
        };

        // when
        let result = apsme.bind_request(request);

        // then
        assert_eq!(result.status, ApsmeBindRequestStatus::IllegalRequest);
    }

    // 2.2.4.3.1
    #[test]
    fn bind_request_from_an_unjoined_device_should_fail() {
        // given
        let mut apsme = Apsme::new();
        let request = ApsmeBindRequest {
            src_address: Address::Extended(0u64),
            src_endpoint: SrcEndpoint::new(10).unwrap_or(SrcEndpoint { value: 0 }),
            cluster_id: 1u16,
            dst_addr_mode: 0u8,
            dst_address: 1u8,
            dst_endpoint: 2u8,
        };

        // when
        let result = apsme.bind_request(request);

        // then
        assert_eq!(result.status, ApsmeBindRequestStatus::IllegalRequest);
    }
    
    // 2.2.4.3.1
    #[test]
    fn bind_request_with_full_table_should_fail() {
        // given
        let mut apsme = Apsme::new();
        apsme.joined_network = Some(Address::Extended(10u64));
        for n in 0..265u64 {
            let request = ApsmeBindRequest {
                src_address: Address::Extended(n),
                src_endpoint: SrcEndpoint::new(10).unwrap_or(SrcEndpoint { value: 0 }),
                cluster_id: 1u16,
                dst_addr_mode: 0u8,
                dst_address: 1u8,
                dst_endpoint: 2u8,
            };
            let _ = apsme.bind_request(request);
        }

        // when
        let request = ApsmeBindRequest {
            src_address: Address::Extended(999u64),
            src_endpoint: SrcEndpoint::new(10).unwrap_or(SrcEndpoint { value: 0 }),
            cluster_id: 1u16,
            dst_addr_mode: 0u8,
            dst_address: 1u8,
            dst_endpoint: 2u8,
        };
        let result = apsme.bind_request(request);

        // then
        assert_eq!(result.status, ApsmeBindRequestStatus::TableFull);
    }

    #[test]
    fn bind_request_with_valid_request_should_succeed() {
        // given
        let mut apsme = Apsme::new();
        apsme.joined_network = Some(Address::Extended(10u64));

        // when
        let request = ApsmeBindRequest {
            src_address: Address::Extended(999u64),
            src_endpoint: SrcEndpoint::new(10).unwrap_or(SrcEndpoint { value: 0 }),
            cluster_id: 1u16,
            dst_addr_mode: 0u8,
            dst_address: 1u8,
            dst_endpoint: 2u8,
        };
        let result = apsme.bind_request(request);

        // then
        assert_eq!(result.status, ApsmeBindRequestStatus::Success);
    }
}
