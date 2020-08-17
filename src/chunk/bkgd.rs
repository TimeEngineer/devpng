// Imports.
use crate::datastream::ColourType;
use std::convert::TryInto;
// Structures.
pub(crate) struct Bkgd<'a>(&'a [u8], ColourType);
// Implementations.
impl<'a> Bkgd<'a> {
    pub(crate) fn from(buf: &'a [u8], colour: ColourType) -> Self {
        Self(buf, colour)
    }
}
impl<'a> std::fmt::Debug for Bkgd<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("bKGD\n");
        s.push_str(&format!("  length: {}\n", length));
        match self.1 {
            ColourType::Indexed => s.push_str(&format!("  palette index: {}\n", self.0[8])),
            ColourType::Greyscale | ColourType::GreyscaleAlpha => s.push_str(&format!(
                "  greyscale: {}\n",
                u16::from_be_bytes(self.0[8..10].try_into().unwrap())
            )),
            ColourType::Truecolour | ColourType::TruecolourAlpha => {
                s.push_str(&format!(
                    "  red: {}\n",
                    u16::from_be_bytes(self.0[8..10].try_into().unwrap())
                ));
                s.push_str(&format!(
                    "  green: {}\n",
                    u16::from_be_bytes(self.0[10..12].try_into().unwrap())
                ));
                s.push_str(&format!(
                    "  blue: {}\n",
                    u16::from_be_bytes(self.0[12..14].try_into().unwrap())
                ));
            }
        }
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
