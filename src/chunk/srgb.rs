// Imports.
use crate::prelude::{Chunk, ChunkMut};
use std::convert::TryInto;
// Structures.
pub struct Srgb<'a>(&'a [u8]);
pub struct SrgbMut<'a>(&'a mut [u8]);
// Implementations.
impl<'a> Chunk for Srgb<'a> {
    fn inner(&self) -> &[u8] {
        self.0
    }
}
impl<'a> ChunkMut for SrgbMut<'a> {
    fn inner(&mut self) -> &mut [u8] {
        self.0
    }
}
impl<'a> Srgb<'a> {
    pub fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> SrgbMut<'a> {
    pub fn from(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Srgb<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("sRGB\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  rendering intent: {}\n", self.0[8]));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
impl<'a> std::fmt::Display for Srgb<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("sRGB\n");
        s.push_str(&format!("  rendering intent: {}\n", self.0[8]));
        write!(f, "{}", s)
    }
}
