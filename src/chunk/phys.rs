// Imports.
use crate::prelude::{Chunk, ChunkMut};
use std::convert::TryInto;
// Structures.
pub struct Phys<'a>(&'a [u8]);
pub struct PhysMut<'a>(&'a mut [u8]);
// Implementations.
impl<'a> Chunk for Phys<'a> {
    fn inner(&self) -> &[u8] {
        self.0
    }
}
impl<'a> ChunkMut for PhysMut<'a> {
    fn inner(&mut self) -> &mut [u8] {
        self.0
    }
}
impl<'a> Phys<'a> {
    pub fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> PhysMut<'a> {
    pub fn from(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Phys<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("pHYs\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!(
            "  ppu x axis: {}\n",
            u32::from_be_bytes(self.0[8..12].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  ppu y axis: {}\n",
            u32::from_be_bytes(self.0[12..16].try_into().unwrap())
        ));
        s.push_str(&format!("  unit specifier: {}\n", self.0[16]));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
impl<'a> std::fmt::Display for Phys<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = format!("pHYs\n");
        s.push_str(&format!(
            "  ppu x axis: {}\n",
            u32::from_be_bytes(self.0[8..12].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  ppu y axis: {}\n",
            u32::from_be_bytes(self.0[12..16].try_into().unwrap())
        ));
        s.push_str(&format!("  unit specifier: {}\n", self.0[16]));
        write!(f, "{}", s)
    }
}
