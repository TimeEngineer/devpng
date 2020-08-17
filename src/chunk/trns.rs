// Imports.
use crate::datastream::ColourType;
use std::convert::TryInto;
// Structures.
pub(crate) struct Trns<'a>(&'a [u8], ColourType);
// Implementations.
impl<'a> Trns<'a> {
    pub(crate) fn from(buf: &'a [u8], colour: ColourType) -> Self {
        Self(buf, colour)
    }
}
impl<'a> std::fmt::Debug for Trns<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("tRNS\n");
        s.push_str(&format!("  length: {}\n", length));
        match self.1 {
            ColourType::Greyscale => s.push_str(&format!(
                "  grey sample value: {}\n",
                u16::from_be_bytes(self.0[8..10].try_into().unwrap())
            )),
            ColourType::Truecolour => {
                s.push_str(&format!(
                    "  red sample value: {}\n",
                    u16::from_be_bytes(self.0[8..10].try_into().unwrap())
                ));
                s.push_str(&format!(
                    "  green sample value: {}\n",
                    u16::from_be_bytes(self.0[10..12].try_into().unwrap())
                ));
                s.push_str(&format!(
                    "  blue sample value: {}\n",
                    u16::from_be_bytes(self.0[12..14].try_into().unwrap())
                ));
            }
            ColourType::Indexed => {
                for (i, v) in self.0.iter().skip(8).take(length).enumerate() {
                    s.push_str(&format!("  alpha for palette index {}: {}\n", i, v));
                }
            }
            _ => unreachable!(),
        }
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
