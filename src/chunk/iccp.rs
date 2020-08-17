// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Iccp<'a>(&'a [u8]);
// Implementations.
impl<'a> Iccp<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Iccp<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;
        let mut base = 8;
        let profil_name_length = self.0[base..].iter().take_while(|x| **x != b'\0').count();
        if profil_name_length + 2 > length {
            return write!(f, "Didn't find null character (iCCP)");
        }
        let profil_name = std::str::from_utf8(&self.0[base..base + profil_name_length]).unwrap();
        base += profil_name_length + 1 + 1;
        let mut s = format!("iCCP\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  profil name: {}\n", profil_name));
        s.push_str(&format!(
            "  compression method: {}\n",
            self.0[9 + profil_name_length]
        ));
        s.push_str(&format!("  compressed profile: [{}]\n", length + 8 - base));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
