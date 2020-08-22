//! # Image

// Imports.
use crate::colour::ColourType;
// Structures.
pub struct Image<'a> {
    pub ncol: usize,
    pub nrow: usize,
    pub depth: u8,
    pub colour: ColourType,
    pub data: &'a mut [u8],
}
// Implementations.
impl<'a> Image<'a> {
    pub fn from(
        ncol: usize,
        nrow: usize,
        depth: u8,
        colour: ColourType,
        data: &'a mut [u8],
    ) -> Self {
        Self {
            ncol,
            nrow,
            depth,
            colour,
            data,
        }
    }
}
impl<'a> std::fmt::Debug for Image<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("size: {}x{}\n", self.nrow, self.ncol));
        s.push_str(&format!("depth: {}\n", self.depth));
        s.push_str(&format!("colour: {:?}\n", self.colour));
        for i in 0..self.nrow {
            s.push_str(&format!(
                "{:02x?}\n",
                &self.data[i * self.ncol..(i + 1) * self.ncol]
            ));
        }
        write!(f, "{}", s)
    }
}
