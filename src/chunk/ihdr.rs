// Imports.
use crate::prelude::{Chunk, ChunkMut, ColourType};
use std::convert::TryInto;
// Structures.
pub struct Ihdr<'a>(&'a [u8]);
pub struct IhdrMut<'a>(&'a mut [u8]);
// Implementations.
impl<'a> Chunk for Ihdr<'a> {
    fn inner(&self) -> &[u8] {
        self.0
    }
}
impl<'a> ChunkMut for IhdrMut<'a> {
    fn inner(&mut self) -> &mut [u8] {
        self.0
    }
}
impl<'a> Ihdr<'a> {
    pub fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
    pub fn width(&self) -> u32 {
        u32::from_be_bytes(self.0[8..12].try_into().unwrap())
    }
    pub fn height(&self) -> u32 {
        u32::from_be_bytes(self.0[12..16].try_into().unwrap())
    }
    pub fn bit_depth(&self) -> u8 {
        self.0[16]
    }
    pub fn colour_type(&self) -> ColourType {
        ColourType::from(self.0[17])
    }
}
impl<'a> IhdrMut<'a> {
    pub fn from(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
    pub fn colour_type(&self) -> ColourType {
        ColourType::from(self.0[17])
    }
}
impl<'a> std::fmt::Debug for Ihdr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("IHDR\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!(
            "  width: {}\n",
            u32::from_be_bytes(self.0[8..12].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  height: {}\n",
            u32::from_be_bytes(self.0[12..16].try_into().unwrap())
        ));
        s.push_str(&format!("  bit depth: {}\n", self.0[16]));
        s.push_str(&format!("  colour type: {}\n", self.0[17]));
        s.push_str(&format!("  compression: {}\n", self.0[18]));
        s.push_str(&format!("  filter: {}\n", self.0[19]));
        s.push_str(&format!("  interlace: {}\n", self.0[20]));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
impl<'a> std::fmt::Display for Ihdr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("IHDR\n");
        s.push_str(&format!(
            "  width: {}\n",
            u32::from_be_bytes(self.0[8..12].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  height: {}\n",
            u32::from_be_bytes(self.0[12..16].try_into().unwrap())
        ));
        s.push_str(&format!("  bit depth: {}\n", self.0[16]));
        s.push_str(&format!("  colour type: {}\n", self.0[17]));
        s.push_str(&format!("  compression: {}\n", self.0[18]));
        s.push_str(&format!("  filter: {}\n", self.0[19]));
        s.push_str(&format!("  interlace: {}\n", self.0[20]));
        write!(f, "{}", s)
    }
}
