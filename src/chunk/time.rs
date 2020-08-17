// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Time<'a>(&'a [u8]);
// Implementations.
impl<'a> Time<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Time<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("tIME\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!(
            "  year: {}\n",
            u16::from_be_bytes(self.0[8..10].try_into().unwrap())
        ));
        s.push_str(&format!("  month: {}\n", self.0[10]));
        s.push_str(&format!("  day: {}\n", self.0[11]));
        s.push_str(&format!("  hour: {}\n", self.0[12]));
        s.push_str(&format!("  minute: {}\n", self.0[13]));
        s.push_str(&format!("  second: {}\n", self.0[14]));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[12..16].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
