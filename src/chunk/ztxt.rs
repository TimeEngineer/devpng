// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Ztxt<'a>(&'a [u8]);
// Implementations.
impl<'a> Ztxt<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Ztxt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut base = 8;
        let keyword_length = self.0[base..].iter().take_while(|x| **x != b'\0').count();
        if keyword_length + 2 > length {
            return write!(f, "Didn't find null character (zTXt)");
        }
        let keyword = std::str::from_utf8(&self.0[base..base + keyword_length]).unwrap();
        base += keyword_length + 1 + 1;
        let mut s = format!("zTXt\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  keyword: {}\n", keyword));
        s.push_str(&format!(
            "  compression method: {}\n",
            self.0[9 + keyword_length]
        ));
        s.push_str(&format!(
            "  compressed text datastream: [{}]\n",
            length + 8 - base
        ));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
