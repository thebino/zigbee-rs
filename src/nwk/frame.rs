use core::mem::transmute;

/// 3.3.1 General NPDU Frame Format
#[derive(Debug)]
pub struct NwkHeader {
    pub frame_control: FrameControl,
    /// 3.3.1.2 Destination Address Field
    pub destination: u16,
    /// 3.3.1.3 Source Address Field
    pub source: u16,
    /// 3.3.1.4 Radius Field
    pub radius: u8,
    /// 3.3.1.5 Sequence Number Field
    pub sequence_number: u8,
    /// 3.3.1.6 Destination IEEE Address Field
    pub destination_ieee: Option<u64>,
    /// 3.3.1.7 Source IEEE Address Field
    pub source_ieee: Option<u64>,
}

/// 3.3.1.1 Frame Control Field
#[derive(Debug)]
pub struct FrameControl(u16);

impl FrameControl {
    pub fn frame_type(&self) -> FrameType {
        // SAFETY: any 2 bit permutation is a valid FrameType
        unsafe { transmute(((self.0 >> 14) & 0b11) as u8) }
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
#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Data = 0b00,
    NwkCommand = 0b01,
    Reserved = 0b10,
    InterPan = 0b11,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
