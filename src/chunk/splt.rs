// Imports.
use crate::crc::{Chunk, ChunkMut};
use std::convert::TryInto;
// Structures.
pub struct Splt<'a>(&'a [u8]);
pub struct SpltMut<'a>(&'a mut [u8]);
// Implementations.
impl<'a> Chunk for Splt<'a> {
    fn inner(&self) -> &[u8] {
        self.0
    }
}
impl<'a> ChunkMut for SpltMut<'a> {
    fn inner(&mut self) -> &mut [u8] {
        self.0
    }
}
impl<'a> Splt<'a> {
    pub fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> SpltMut<'a> {
    pub fn from(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Splt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let palette_name_length = self.0[8..].iter().take_while(|x| **x != b'\0').count();
        if palette_name_length + 2 > length {
            return write!(f, "Didn't find null character (sPLT)");
        }
        let palette_name = std::str::from_utf8(&self.0[8..8 + palette_name_length]).unwrap();
        let sample_depth = self.0[9 + palette_name_length];
        let mut s = format!("sPLT\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  palette name: {}\n", palette_name));
        s.push_str(&format!("  sample depth: {}\n", sample_depth));
        match sample_depth {
            8 => {
                if palette_name_length + 8 != length {
                    return write!(f, "Length doesn't match (sPLT)");
                }
                s.push_str(&format!("  red: {}\n", self.0[10 + palette_name_length]));
                s.push_str(&format!("  green: {}\n", self.0[11 + palette_name_length]));
                s.push_str(&format!("  blue: {}\n", self.0[12 + palette_name_length]));
                s.push_str(&format!("  alpha: {}\n", self.0[13 + palette_name_length]));
                s.push_str(&format!(
                    "  frequency: {}\n",
                    u16::from_be_bytes(
                        self.0[14 + palette_name_length..16 + palette_name_length]
                            .try_into()
                            .unwrap()
                    )
                ));
            }
            16 => {
                if palette_name_length + 12 != length {
                    return write!(f, "Length doesn't match (sPLT)");
                }
                s.push_str(&format!(
                    "  red: {}\n",
                    u16::from_be_bytes(
                        self.0[10 + palette_name_length..12 + palette_name_length]
                            .try_into()
                            .unwrap()
                    )
                ));
                s.push_str(&format!(
                    "  green: {}\n",
                    u16::from_be_bytes(
                        self.0[12 + palette_name_length..14 + palette_name_length]
                            .try_into()
                            .unwrap()
                    )
                ));
                s.push_str(&format!(
                    "  blue: {}\n",
                    u16::from_be_bytes(
                        self.0[14 + palette_name_length..16 + palette_name_length]
                            .try_into()
                            .unwrap()
                    )
                ));
                s.push_str(&format!(
                    "  alpha: {}\n",
                    u16::from_be_bytes(
                        self.0[16 + palette_name_length..18 + palette_name_length]
                            .try_into()
                            .unwrap()
                    )
                ));
                s.push_str(&format!(
                    "  frequency: {}\n",
                    u16::from_be_bytes(
                        self.0[18 + palette_name_length..20 + palette_name_length]
                            .try_into()
                            .unwrap()
                    )
                ));
            }
            _ => return write!(f, "Sample depth not covered (sPLT)"),
        }
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
