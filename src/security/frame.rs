use crate::{address::IeeeAddress, impl_pack_bytes};

/// Auxiliary Frame Header Format
///
/// See Section 4.5.1.
pub struct AuxFrameHeader {
    pub security_control: SecurityControl,
    pub frame_counter: u32,
    pub source_address: Option<IeeeAddress>,
    pub key_sequence_numner: Option<u8>,
}

impl_pack_bytes! {
    #[derive(Clone, Copy )]
    pub struct SecurityControl(pub u8);
}
