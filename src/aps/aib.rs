use heapless::Vec;


/// 2.2.7.2 - AIB (APS Information Base Attributes)
pub struct ApsInformationBase {
    attributes: Vec<AIBAttribute, 265>
}

impl ApsInformationBase {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new()
        }
    }
    pub fn get_attribute(&self, id: u8) -> Option<&AIBAttribute> {
        self.attributes.iter().find(|attr| attr.id() == id)
    }
    pub fn write_attribute_value(&self, _id: u8, _value: AIBAttributeValue) -> Result<(), &'static str> {
        Ok(())
    }
}

impl Default for ApsInformationBase {
    fn default() -> Self {
        Self::new()
    }
}

pub type AIBAttributeValue = [u8; 8];


#[derive(Debug, PartialEq)]
pub enum AIBAttribute {
    ApsBindingTable,
    ApsDesignatedCoordinator(bool),
    ApsChannelMaskList(u8),
    ApsUseExtendedPANID(u64),
    ApsGroupTable(u8),
    ApsNonmemberRadius(u8),
    ApsUseInsecureJoin(bool),
    ApsInterframeDelay(u8),
    ApsLastChannelEnergy(u8),
    ApsLastChannelFailureRate(u8),
    ApsChannelTimer(u8),
    ApsMaxWindowSize(u8),
    ApsParentAnnounceTimer(u8),
}

impl AIBAttribute {
    pub fn id(&self) -> u8 {
        match self {
            // 0xc0 removed in 2007
            AIBAttribute::ApsBindingTable => 0xc1,
            AIBAttribute::ApsDesignatedCoordinator(_) => 0xc2,
            AIBAttribute::ApsChannelMaskList(_) => 0xc3,
            AIBAttribute::ApsUseExtendedPANID(_) => 0xc4,
            AIBAttribute::ApsGroupTable(_)=> 0xc5,
            AIBAttribute::ApsNonmemberRadius(_)=> 0xc6,
            // 0xc7 removed (ApsIpIdPermissionsConfig)
            AIBAttribute::ApsUseInsecureJoin(_)=> 0xc8,
            AIBAttribute::ApsInterframeDelay(_)=> 0xc9,
            AIBAttribute::ApsLastChannelEnergy(_)=> 0xca,
            AIBAttribute::ApsLastChannelFailureRate(_)=> 0xcb,
            AIBAttribute::ApsChannelTimer(_)=> 0xcc,
            AIBAttribute::ApsMaxWindowSize(_)=> 0xcd,
            AIBAttribute::ApsParentAnnounceTimer(_)=> 0xce,
            // 0x0500 to 0x05ff reserved for custom AIBs 
        }
    }
    pub fn length(&self) -> u8 {
        match self {
            AIBAttribute::ApsBindingTable => 0u8,
            _ => 0u8
        }
    }
    pub fn value(&self) -> [u8; 8] {
        match self {

           AIBAttribute::ApsUseExtendedPANID(id) => id.to_be_bytes(),
            _ => [0u8; 8]
        }
    }
}


