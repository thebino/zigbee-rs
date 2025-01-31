pub type NwkAddress = u16;

pub mod macros {
    macro_rules! bitfield_bits {
    () => {
        heapless::FnvIndexSet::new()
    };
    ($ty: ty; $($x: expr),+ $(,)?) => {{
        const CAPACITY: usize =<$ty>::COUNT.next_power_of_two();
        let mut bits = heapless::FnvIndexSet::<$ty, CAPACITY>::new();
        $(
            let _ = bits.insert($x);
        )+
        bits
    }};
}
    pub(crate) use bitfield_bits;
}

use core::fmt;

use crate::impl_pack_bytes;

impl_pack_bytes! {
    /// 16-bit network address
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ShortAddress(pub u16);
}

impl fmt::Debug for ShortAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ShortAddress(0x{:04x})", self.0)
    }
}

impl_pack_bytes! {
    /// 64-bit network address
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct IeeeAddress(pub u64);
}

impl fmt::Debug for IeeeAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IeeeAddress(0x{:016x})", self.0)
    }
}
