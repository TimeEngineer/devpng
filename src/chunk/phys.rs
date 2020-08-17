// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Phys<'a>(&'a [u8]);
// Implementations.
impl<'a> Phys<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
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
            u32::from_be_bytes(self.0[17..21].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
