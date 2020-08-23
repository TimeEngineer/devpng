// Imports.
use crate::prelude::{Chunk, ChunkMut, ColourType};
use std::convert::TryInto;
// Structures.
pub struct Sbit<'a>(&'a [u8], ColourType);
pub struct SbitMut<'a>(&'a mut [u8], ColourType);
// Implementations.
impl<'a> Chunk for Sbit<'a> {
    fn inner(&self) -> &[u8] {
        self.0
    }
}
impl<'a> ChunkMut for SbitMut<'a> {
    fn inner(&mut self) -> &mut [u8] {
        self.0
    }
}
impl<'a> Sbit<'a> {
    pub fn from(buf: &'a [u8], colour: ColourType) -> Self {
        Self(buf, colour)
    }
}
impl<'a> SbitMut<'a> {
    pub fn from(buf: &'a mut [u8], colour: ColourType) -> Self {
        Self(buf, colour)
    }
}
impl<'a> std::fmt::Debug for Sbit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("sBIT\n");
        s.push_str(&format!("  length: {}\n", length));
        match self.1 {
            ColourType::Greyscale => {
                s.push_str(&format!("  significant greyscale bits: {}\n", self.0[8]))
            }
            ColourType::RGB | ColourType::Indexed => {
                s.push_str(&format!("  significant red bits: {}\n", self.0[8]));
                s.push_str(&format!("  significant green bits: {}\n", self.0[9]));
                s.push_str(&format!("  significant blue bits: {}\n", self.0[10]));
            }
            ColourType::GreyscaleAlpha => {
                s.push_str(&format!("  significant greyscale bits: {}\n", self.0[8]));
                s.push_str(&format!("  significant alpha bits: {}\n", self.0[9]));
            }
            ColourType::RGBA => {
                s.push_str(&format!("  significant red bits: {}\n", self.0[8]));
                s.push_str(&format!("  significant green bits: {}\n", self.0[9]));
                s.push_str(&format!("  significant blue bits: {}\n", self.0[10]));
                s.push_str(&format!("  significant alpha bits: {}\n", self.0[11]));
            }
        }
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
impl<'a> std::fmt::Display for Sbit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("sBIT\n");
        match self.1 {
            ColourType::Greyscale => {
                s.push_str(&format!("  significant greyscale bits: {}\n", self.0[8]))
            }
            ColourType::RGB | ColourType::Indexed => {
                s.push_str(&format!("  significant red bits: {}\n", self.0[8]));
                s.push_str(&format!("  significant green bits: {}\n", self.0[9]));
                s.push_str(&format!("  significant blue bits: {}\n", self.0[10]));
            }
            ColourType::GreyscaleAlpha => {
                s.push_str(&format!("  significant greyscale bits: {}\n", self.0[8]));
                s.push_str(&format!("  significant alpha bits: {}\n", self.0[9]));
            }
            ColourType::RGBA => {
                s.push_str(&format!("  significant red bits: {}\n", self.0[8]));
                s.push_str(&format!("  significant green bits: {}\n", self.0[9]));
                s.push_str(&format!("  significant blue bits: {}\n", self.0[10]));
                s.push_str(&format!("  significant alpha bits: {}\n", self.0[11]));
            }
        }
        write!(f, "{}", s)
    }
}
