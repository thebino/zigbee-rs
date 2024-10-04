use core::{
    fmt::Debug,
    mem::{self},
};

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

const PAYLOAD_CAP: usize = 128;

pub struct ShortAddress(u16);

impl Debug for ShortAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ShortAddress({:#x})", self.0)
    }
}

impl PackBytes for ShortAddress {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        Some(Self(u16::unpack_from_iter(src)?))
    }
}

pub struct IeeeAddress(u64);

impl Debug for IeeeAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IeeeAddress({:#x})", self.0)
    }
}

impl PackBytes for IeeeAddress {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        Some(Self(u64::unpack_from_iter(src)?))
    }
}

#[derive(Debug)]
pub struct NwkFrame {
    pub header: NwkHeader,
    pub payload: Vec<u8, PAYLOAD_CAP>,
}

impl PackBytes for NwkFrame {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let mut src = src.into_iter();
        Some(Self {
            header: PackBytes::unpack_from_iter(&mut src)?,
            payload: src.collect(),
        })
    }
}

/// 3.3.1 General NPDU Frame Format
#[derive(Debug)]
pub struct NwkHeader {
    pub frame_control: FrameControl,
    /// See Section 3.3.1.2.
    pub destination: ShortAddress,
    /// See Section 3.3.1.3.
    pub source: ShortAddress,
    /// See Section 3.3.1.4.
    pub radius: u8,
    /// See Section 3.3.1.5.
    pub sequence_number: u8,
    /// Set only if [`FrameControl::destination_ieee_flag`] is `true`.
    /// See Section 3.3.1.6.
    pub destination_ieee: Option<IeeeAddress>,
    /// Set only if [`FrameControl::source_ieee_flag`] is `true`.
    /// See Section 3.3.1.7.
    pub source_ieee: Option<IeeeAddress>,
    /// Set only if [`FrameControl::multicast_flag`] is `true`.
    /// See Section 3.3.1.8.
    pub multicast_control: Option<MulticastControl>,
    /// Set only if [`FrameControl::source_flag`] is `true`.
    /// See Section 3.3.1.9.
    pub source_route_subframe: Option<SourceRouteSubframe>,
}

impl PackBytes for NwkHeader {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let mut src = src.into_iter();
        let frame_control = FrameControl::unpack_from_iter(&mut src)?;
        Some(Self {
            destination: PackBytes::unpack_from_iter(&mut src)?,
            source: PackBytes::unpack_from_iter(&mut src)?,
            radius: src.next()?,
            sequence_number: src.next()?,
            destination_ieee: frame_control
                .destination_ieee_flag()
                .then(|| PackBytes::unpack_from_iter(&mut src))
                .flatten(),
            source_ieee: frame_control
                .source_ieee_flag()
                .then(|| PackBytes::unpack_from_iter(&mut src))
                .flatten(),
            multicast_control: frame_control
                .multicast_flag()
                .then(|| MulticastControl::unpack_from_iter(&mut src))
                .flatten(),
            source_route_subframe: frame_control
                .source_flag()
                .then(|| SourceRouteSubframe::unpack_from_iter(src))
                .flatten(),
            frame_control,
        })
    }
}

impl PackBytes for u16 {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let buf: Vec<u8, 2> = src.into_iter().take(2).collect();
        Some(u16::from_be_bytes(buf.into_array().unwrap()))
    }
}

impl PackBytes for u64 {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let buf: Vec<u8, 8> = src.into_iter().take(8).collect();
        Some(u64::from_be_bytes(buf.into_array().unwrap()))
    }
}

impl PackBytes for FrameControl {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        Some(Self(u16::unpack_from_iter(src)?))
    }
}

/// 3.3.1.1 Frame Control Field
pub struct FrameControl(u16);

impl Debug for FrameControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FrameControl")
            .field("frame_type", &self.frame_type())
            .field("protocol_version", &self.protocol_version())
            .field("discover_route", &self.discover_route())
            .field("multicast_flag", &self.multicast_flag())
            .field("security_flag", &self.security_flag())
            .field("source_flag", &self.source_flag())
            .field("destination_ieee_flag", &self.destination_ieee_flag())
            .field("source_ieee_flag", &self.source_ieee_flag())
            .field("end_device_initiator", &self.end_device_initiator())
            .finish()
    }
}

impl FrameControl {
    pub fn frame_type(&self) -> FrameType {
        // SAFETY: any 2 bit permutation is a valid FrameType
        unsafe { mem::transmute(((self.0 >> 14) & 0b11) as u8) }
    }

    /// See Section 3.3.1.1.2.
    pub fn protocol_version(&self) -> u8 {
        ((self.0 >> 10) & 0b1111) as u8
    }

    /// See Section 3.3.1.1.3.
    pub fn discover_route(&self) -> DiscoverRoute {
        DiscoverRoute::from_u8(((self.0 >> 8) & 0b11) as u8)
    }

    /// See Section 3.3.1.1.4.
    pub fn multicast_flag(&self) -> bool {
        ((self.0 >> 7) & 0b1) != 0
    }

    /// The security sub-field shall have a value of 1 if, and only if, the
    /// frame is to have NWK security operations enabled. If security for
    /// this frame is implemented at another layer or disabled entirely,
    /// it shall have a value of 0.
    ///
    /// See Section 3.3.1.1.5.
    pub fn security_flag(&self) -> bool {
        ((self.0 >> 6) & 0b1) != 0
    }

    /// The source route sub-field shall have a value of 1 if and only if a source route subframe
    /// is present in the NWK header. If the source route subframe is not present, the source route
    /// sub-field shall have a value of 0.
    ///
    /// See Section 3.3.1.1.6.
    pub fn source_flag(&self) -> bool {
        ((self.0 >> 5) & 0b1) != 0
    }

    /// The destination IEEE address sub-field shall have a value of 1 if, and only if, the NWK
    /// header is to include the full IEEE address of the destination.
    ///
    /// See Section 3.3.1.1.7.
    pub fn destination_ieee_flag(&self) -> bool {
        ((self.0 >> 4) & 0b1) != 0
    }

    /// The source IEEE address sub-field shall have a value of 1 if, and only if, the NWK
    /// header is to include the full IEEE address of the source device.
    ///
    /// See Section 3.3.1.1.8.
    pub fn source_ieee_flag(&self) -> bool {
        ((self.0 >> 3) & 0b1) != 0
    }

    /// See Section 3.3.1.1.9.
    pub fn end_device_initiator(&self) -> bool {
        ((self.0 >> 2) & 0b1) != 0
    }
}

/// 3.3.1.1.1 Frame Type Sub-Field
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Data = 0b00,
    NwkCommand = 0b01,
    Reserved = 0b10,
    InterPan = 0b11,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscoverRoute {
    Suppress,
    Enable,
    Reserved,
}

impl DiscoverRoute {
    fn from_u8(b: u8) -> Self {
        match b {
            0x00 => Self::Suppress,
            0x01 => Self::Enable,
            _ => Self::Reserved,
        }
    }
}

/// 3.3.1.8 Multicast Control Field
pub struct MulticastControl(u8);

impl Debug for MulticastControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MulticastControl")
            .field("multicast_mode", &self.multicast_mode())
            .field("non_member_radius", &self.non_member_radius())
            .field("max_member_radius", &self.max_member_radius())
            .finish()
    }
}

impl PackBytes for MulticastControl {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        Some(Self(src.into_iter().next()?))
    }
}

impl MulticastControl {
    /// See Section 3.3.1.8.1.
    pub fn multicast_mode(&self) -> MulticastMode {
        match (self.0 >> 6) & 0b11 {
            0b00 => MulticastMode::NonMemberMode,
            0b01 => MulticastMode::MemberMode,
            _ => MulticastMode::Reserved,
        }
    }

    /// See Section 3.3.1.8.2.
    pub fn non_member_radius(&self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    /// The maximum value of the [`MulticastControl::non_member_radius`] sub-field for this frame.
    /// See Section 3.3.1.8.3.
    pub fn max_member_radius(&self) -> u8 {
        self.0 & 0b11
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MulticastMode {
    NonMemberMode = 0b00,
    MemberMode = 0b01,
    Reserved,
}

const RELAY_LIST_CAP: usize = 16;

#[derive(Debug)]
pub struct SourceRouteSubframe {
    /// Indicates the number of relays contained in [`SourceRouteSubframe::relay_list`].
    ///
    /// See Section 3.3.1.9.1.
    pub relay_count: u8,
    /// Indicates the index of the next relay in [`SourceRouteSubframe::relay_list`] to
    /// which the packet will be transmitted.
    ///
    /// See Section 3.3.1.9.2.
    pub relay_index: u8,
    /// List of relay addresses from closest to the destination to closest to the originator.
    ///
    /// See Section 3.3.1.9.2.
    pub relay_list: Vec<u8, RELAY_LIST_CAP>,
}

impl PackBytes for SourceRouteSubframe {
    fn unpack_from_iter(src: impl IntoIterator<Item = u8>) -> Option<Self> {
        let mut src = src.into_iter();
        Some(Self {
            relay_count: src.next()?,
            relay_index: src.next()?,
            relay_list: src.collect(),
        })
    }
}

//impl PackedStructSlice for SourceRouteSubframe {
//    fn pack_to_slice(&self, output: &mut [u8]) -> packed_struct::PackingResult<()> {
//        let mut output = output.iter_mut();
//        *output.next().ok_or(PackingError::BufferTooSmall)? = self.relay_count;
//        *output.next().ok_or(PackingError::BufferTooSmall)? = self.relay_index;
//        for l in &self.relay_list {
//            *output.next().ok_or(PackingError::BufferTooSmall)? = *l
//        }

//        Ok(())
//    }

//    fn unpack_from_slice(src: &[u8]) -> packed_struct::PackingResult<Self> {
//        let relay_count = src[0];
//        let relay_index = src[1];
//        let relay_list = &src[2..];
//        Ok(Self {
//            relay_count,
//            relay_index,
//            relay_list: Vec::from_slice(relay_list).map_err(|_| PackingError::BufferTooSmall)?,
//        })
//    }

//    fn packed_bytes_size(opt_self: Option<&Self>) -> packed_struct::PackingResult<usize> {
//        opt_self
//            .map(|s| mem::size_of::<u8>() * 2 + s.relay_list.len())
//            .ok_or(PackingError::InstanceRequiredForSize)
//    }
//}

#[cfg(test)]
mod tests {
    const EXAMPLE_PAYLOAD: &str = "0912fcffda8601de8759f2feff142e842806a546008759f2feff142e840061d1e5ca8d4596ea43dac1724521898dc11684523abaf4db";
    const EXAMPLE_PAYLOAD_2: &str = "0912fcffc38101208f119c775238c1a42871f815008f119c775238c1a400f02eb0f15490f73fd5976e454332249efae18cee9c";

    use super::*;

    #[test]
    fn multicast_control_multicast_mode() {
        let multicast_control = MulticastControl(0xff);
        let mode = multicast_control.multicast_mode();
        assert_eq!(mode, MulticastMode::Reserved);
    }

    #[test]
    fn source_route_subframe_pack() {
        let got =
            SourceRouteSubframe::unpack_from_slice(&[0x04, 0x03, 0xff, 0xff, 0xff, 0xff]).unwrap();

        assert_eq!(got.relay_count, 0x04);
        assert_eq!(got.relay_index, 0x03);
        assert_eq!(got.relay_list, &[0xff, 0xff, 0xff, 0xff]);
    }

    //#[test]
    //fn unpack() {
    //    let src = [1, 2, 3, 4, 5, 6, 7, 8];
    //    let mut src = src.into_iter();
    //    let s = src.by_ref();
    //    s.take(2).collect::<Vec<&i32, 2>>();
    //    assert!(false, "{:?}", s.collect::<Vec<&i32, 10>>())
    //}

    #[test]
    fn unpack_frame_control() {
        let raw = [0b00111101u8, 0b01010100u8];

        let frame_control = FrameControl::unpack_from_slice(&raw).unwrap();
        assert_eq!(frame_control.frame_type(), FrameType::Data);
        assert_eq!(frame_control.protocol_version(), 0b1111u8);
        assert_eq!(frame_control.discover_route(), DiscoverRoute::Enable);
        assert!(!frame_control.multicast_flag());
        assert!(frame_control.security_flag());
        assert!(!frame_control.source_flag());
        assert!(frame_control.destination_ieee_flag());
        assert!(!frame_control.source_ieee_flag());
        assert!(frame_control.end_device_initiator());
    }

    #[test]
    fn unpack_nwk_frame() {
        let mut raw = [0u8; EXAMPLE_PAYLOAD.len() / 2];
        hex::decode_to_slice(EXAMPLE_PAYLOAD, &mut raw).unwrap();

        let frame = NwkFrame::unpack_from_slice(&raw);
        assert!(false, "frame {:#?}", frame)
    }
}
