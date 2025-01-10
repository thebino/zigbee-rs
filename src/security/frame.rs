//! Security Frame Formats
use crate::impl_pack_bytes;
use crate::types::IeeeAddress;

/// Auxiliary Frame Header Format
///
/// See Section 4.5.1.
pub struct AuxFrameHeader {
    /// Security control
    pub security_control: SecurityControl,
    /// Frame counter
    pub frame_counter: u32,
    /// Set only if [`SecurityControl::extended_nonce`] is `true`.
    pub source_address: Option<IeeeAddress>,
    /// Set only if [`SecurityControl::key_identifier`] is `1`.
    pub key_sequence_numner: Option<u8>,
}

impl_pack_bytes! {
    /// Security Control
    ///
    /// See Section 4.5.1.1.
    #[derive(Clone, Copy)]
    pub struct SecurityControl(pub u8);
}
