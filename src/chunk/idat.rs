// Imports.
use crate::prelude::{Chunk, ChunkMut};
use std::convert::TryInto;
// Strucutures.
pub struct Idat<'a>(&'a [u8]);
pub struct IdatMut<'a>(&'a mut [u8]);
// Implementations.
impl<'a> Chunk for Idat<'a> {
    fn inner(&self) -> &[u8] {
        self.0
    }
}
impl<'a> ChunkMut for IdatMut<'a> {
    fn inner(&mut self) -> &mut [u8] {
        self.0
    }
}
impl<'a> Idat<'a> {
    pub fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> IdatMut<'a> {
    pub fn from(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Idat<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("IDAT\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  idat: [{}]\n", length));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
impl<'a> std::fmt::Display for Idat<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("IDAT\n");
        s.push_str(&format!("  idat: [{}]\n", length));
        write!(f, "{}", s)
    }
}
