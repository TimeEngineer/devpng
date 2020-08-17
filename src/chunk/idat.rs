// Imports.
use std::convert::TryInto;
// Strucutures.
pub(crate) struct Idat<'a>(&'a [u8]);
// Implementations.
impl<'a> Idat<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
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
