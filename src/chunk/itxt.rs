// Imports.
use std::convert::TryInto;
// Structures.
pub(crate) struct Itxt<'a>(&'a [u8]);
// Implementations.
impl<'a> Itxt<'a> {
    pub(crate) fn from(buf: &'a [u8]) -> Self {
        Self(buf)
    }
}
impl<'a> std::fmt::Debug for Itxt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let length = u32::from_be_bytes(self.0[0..4].try_into().unwrap()) as usize;

        let mut base = 8;
        let keyword_length = self.0[base..].iter().take_while(|x| **x != b'\0').count();
        if keyword_length + 3 > length {
            return write!(f, "Didn't find null character 0 (iTXt)");
        }
        let keyword = std::str::from_utf8(&self.0[base..base + keyword_length]).unwrap();
        base += keyword_length + 1 + 1 + 1;
        let language_tag_length = self.0[base..].iter().take_while(|x| **x != b'\0').count();
        if keyword_length + language_tag_length + 4 > length {
            return write!(f, "Didn't find null character 1 (iTXt)");
        }
        let language_tag = std::str::from_utf8(&self.0[base..base + language_tag_length]).unwrap();
        base += language_tag_length + 1;
        let translated_keyword_length = self.0[base..].iter().take_while(|x| **x != b'\0').count();
        if keyword_length + language_tag_length + translated_keyword_length + 5 > length {
            return write!(f, "Didn't find null character 2 (iTXt)");
        }
        let translated_keyword =
            std::str::from_utf8(&self.0[base..base + translated_keyword_length]).unwrap();
        base += translated_keyword_length + 1;

        let mut s = format!("iTXt\n");
        s.push_str(&format!("  length: {}\n", length));
        s.push_str(&format!("  keyword: {}\n", keyword));
        s.push_str(&format!(
            "  compression flag: {}\n",
            self.0[9 + keyword_length]
        ));
        s.push_str(&format!(
            "  compression method: {}\n",
            self.0[10 + keyword_length]
        ));
        s.push_str(&format!("  language tag: {}\n", language_tag));
        s.push_str(&format!("  translated_keyword: {}\n", translated_keyword));
        s.push_str(&format!("  text: [{}]\n", length + 8 - base));
        s.push_str(&format!(
            "  crc: 0x{:08X}\n",
            u32::from_be_bytes(self.0[8 + length..].try_into().unwrap())
        ));
        write!(f, "{}", s)
    }
}
