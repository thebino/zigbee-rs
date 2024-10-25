//! # zigbee
//!
//! ZigBee is a protocol stack based on the ZigBee specification 22 1.0
//!
#![no_std]
mod spec;
pub use spec::*;

pub(crate) mod parse;
