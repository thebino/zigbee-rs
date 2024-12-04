//! Implements the ZigBee protocol stack in `no-std` based on the [ZigBee specification]
//!
//! [ZigBee specification]: https://csa-iot.org/wp-content/uploads/2022/01/docs-05-3474-22-0csg-zigbee-specification-1.pdf
//!
#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

/// This aps (application support sub-layer) module provides an interface between the `nwk`
/// (Network layer) and the `apl` (Application layer) through a general set of services that are
/// used by both the `zdo` (Zigbee device object) and the application objects.
pub mod aps;

