// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Plte<'a>(&'a [u8]);
// Implementations.
impl<'a> Plte<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Plte<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("PLTE\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  palette: [{}][3]\n", length / 3));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
