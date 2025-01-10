use core::iter::FromIterator;

use heapless::Vec;

pub trait PackBytes
where
    Self: Sized,
{
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self>;
    fn unpack_from_slice(src: &[u8]) -> Option<Self> {
        Self::unpack_from_iter(src.iter().cloned())
    }
}

impl PackBytes for u8 {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        src.into_iter().next()
    }
}

impl PackBytes for i8 {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        src.into_iter().next().map(|b| b as i8)
    }
}

macro_rules! impl_primitive {
    ($ty:ty, $sz:literal) => {
        impl PackBytes for $ty {
            fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
                let buf: Vec<u8, $sz> = src.into_iter().take($sz).collect();
                Some(<$ty>::from_le_bytes(buf.into_array().unwrap()))
            }
        }
    };
}

impl_primitive!(u16, 2);
impl_primitive!(u32, 4);
impl_primitive!(u64, 8);
impl_primitive!(i16, 2);
impl_primitive!(i32, 4);
impl_primitive!(i64, 8);

impl<const N: usize> PackBytes for Vec<u8, N> {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        Some(FromIterator::from_iter(src))
    }
}

#[macro_export]
macro_rules! impl_pack_bytes {
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident($vt:vis $ty:ty);
    ) => {
        $(#[$m])*
        $v struct $name($vt $ty);

        impl $crate::common::parse::PackBytes for $name {
            fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
                Some(Self($crate::common::parse::PackBytes::unpack_from_iter(src)?))
            }
        }
    };
    (
        $(#[$m:meta])*
        $v:vis struct $name:ident {
            $(
                $(#[doc = $doc:literal])*
                $(#[transparent($tp_ty:ty)])?
                $(#[collect($cl_ty:ty)])?
                $vf:vis $field_name:ident: $field_ty:ty
            ),+
            $(,)?
        }
    ) => {
        $(#[$m])*
        $v struct $name{
            $(
                $(#[doc = $doc])*
                $vf $field_name: $field_ty
            ),+
        }

        impl $crate::common::parse::PackBytes for $name {
            fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
                use core::iter::FromIterator;
                let mut src = src.into_iter();
                Some(Self {
                    $(
                        $(
                            $field_name: <$tp_ty>::unpack_from_iter(&mut src)?
                        )?
                        $(
                            $field_name: <$cl_ty>::from_iter(src)
                        )?
                    ),+
                })
            }
        }
    }
}
