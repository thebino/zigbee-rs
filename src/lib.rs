//! Implements the ZigBee protocol stack in `no-std` based on the [ZigBee specification]
//!
//! [ZigBee specification]: https://csa-iot.org/wp-content/uploads/2022/01/docs-05-3474-22-0csg-zigbee-specification-1.pdf
//!
#![no_std]
mod spec;
pub use spec::*;

pub(crate) mod parse;
