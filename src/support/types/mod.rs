//! Bounded address
//! 2.2.4.1.1
pub struct SrcEndpoint {
    value: u8
}

impl SrcEndpoint {
    pub fn new(value: u8) -> Result<Self, String> {
        if value >= 2 && value <= 100 {
            Ok(SrcEndpoint { value })
        } else {
            Err(format!("Value {} is not within the valid range 0x00 – 0xfe", value))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undersized_value_should_fail() {
        let src_endpoint = SrcEndpoint::new(1);

        assert!(src_endpoint.is_err());
    }

    #[test]
    fn oversized_value_should_fail() {
        let src_endpoint = SrcEndpoint::new(101);

        assert!(src_endpoint.is_err());
    }
}

