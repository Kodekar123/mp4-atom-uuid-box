use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uuid {
    pub extended_type: [u8; 16],
}

impl Atom for Uuid {
    const KIND: FourCC = FourCC::new(b"uuid");

    fn decode_body<B: Buf>(buf: &mut B) -> Result<Self> {
        let extended_type = <[u8; 16]>::decode(buf)?;

        Ok(Uuid { extended_type })
    }

    fn encode_body<B: BufMut>(&self, buf: &mut B) -> Result<()> {
        self.extended_type.encode(buf)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_round_trip() -> Result<()> {
        let mut buf = Vec::<u8>::new();

        let original = Uuid {
            extended_type: [
                0xD8, 0xFE, 0xC3, 0xD6, 0x1B, 0x0E, 0x48, 0x3C, 0x92, 0x97, 0x58, 0x28, 0x87, 0x7E,
                0xC4, 0x81,
            ],
        };

        original.encode_body(&mut buf)?;

        let mut read_original_slice = &buf[..];
        let decoded = Uuid::decode_body(&mut read_original_slice)?;

        assert_eq!(original, decoded);
        Ok(())
    }
}
