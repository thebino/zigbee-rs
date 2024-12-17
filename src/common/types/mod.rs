pub struct IeeeAddress(u64);

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
