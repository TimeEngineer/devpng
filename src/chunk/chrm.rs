// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Chrm<'a>(&'a [u8]);
// Implementations.
impl<'a> Chrm<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Chrm<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut s = format!("cHRM\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!(
            "  white point x: {}\n",
            u32::from_be_bytes(self.0[8..12].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  white point y: {}\n",
            u32::from_be_bytes(self.0[12..16].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  red x: {}\n",
            u32::from_be_bytes(self.0[16..20].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  red y: {}\n",
            u32::from_be_bytes(self.0[20..24].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  green x: {}\n",
            u32::from_be_bytes(self.0[24..28].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  green y: {}\n",
            u32::from_be_bytes(self.0[28..32].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  blue x: {}\n",
            u32::from_be_bytes(self.0[32..36].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  blue y: {}\n",
            u32::from_be_bytes(self.0[36..40].try_into().unwrap())
        ));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[40..44].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
