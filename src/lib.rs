//! Implements the ZigBee protocol stack in `no-std` based on the [ZigBee specification R22 1.0]
//!
//! [ZigBee specification R22 1.0]: https://csa-iot.org/wp-content/uploads/2022/01/docs-05-3474-22-0csg-zigbee-specification-1.pdf
//!
//! The crate needs some peripherals from the underlying platform and some persistency during the
//! setup.
//!
//! **This is how it could look like in the future**
//!
//! ```rust
//! let zigbee_device = zigbee::init(zigbee::Config { radio_channel: 11, ..Default::default() });
//!
//! let temperature_sensor = zigbee;
//! let _ = zigbee_device.register(temperature_sensor);
//! let available_networks: Vec<zigbee::ZigBeeNetwork> = zigbee_device.scanning_networks();
//! let parent_device = available_networks[0].device[0];
//! let response = parent_device.request_to_join();
//! ```
//!
//! # ESP32 & nRF support
//!
//! This crate is currently only supporting devices in the Espressif ecosystem, but presumative
//! this will expand to nordics nRF series.
//!
#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

/// The `application support sub-layer` provides an interface between the `Network layer` and the `Application layer`. 
pub mod aps;

/// 2.3 Application framework
pub mod apl;

mod zdo;
pub use zdo::config::Config;
pub use zdo::config::DiscoveryType;
pub use apl::descriptors::LogicalType;
pub use zdo::ZigbeeDevice;
pub use zdo::ZigBeeNetwork;


mod common;

/// Initialize a new zigbee device with the default configuartion.
///
/// Initialize a new zigbee device with a configuration
pub fn init(config: zdo::config::Config) -> ZigbeeDevice {
    let device = ZigbeeDevice::default();
    device.configure(config);

    device
}

