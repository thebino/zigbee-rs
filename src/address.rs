use core::fmt;

use crate::impl_pack_bytes;

impl_pack_bytes! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ShortAddress(pub u16);
}

impl fmt::Debug for ShortAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ShortAddress(0x{:04x})", self.0)
    }
}

impl_pack_bytes! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct IeeeAddress(pub u64);
}

impl fmt::Debug for IeeeAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IeeeAddress(0x{:016x})", self.0)
    }
}
