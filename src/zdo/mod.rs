use config::Config;

pub mod config;

pub struct ZigbeeDevice {
    config: Config,
} 


pub struct ZigBeeNetwork {

}



impl ZigbeeDevice {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }
    pub fn configure(&self, _config: Config) {

    }

    pub fn start_device_discovery(&self) {
        match self.config.device_discovery_type {
            config::DiscoveryType::IEEE => {
                todo!()
                // TODO: send IEEE address request as unicast to a particular device 
            },
            config::DiscoveryType::NWK => {
                todo!()
                // TODO: send NWK address request as broadcast with the known IEEE address as data payload
            },
        }
    }
    pub fn start_service_discovery(&self) {}
}



impl Default for ZigbeeDevice {
    fn default() -> Self {
        Self::new()
    }
}

