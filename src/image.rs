//! # Image

// Imports.
use crate::prelude::ColourType;
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
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            ncol: 0,
            nrow: 0,
            depth: 8,
            colour: ColourType::RGB,
            data,
        }
    }
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
    pub fn ncol(&self) -> usize {
        self.ncol
    }
    pub fn nrow(&self) -> usize {
        self.nrow
    }
    pub fn depth(&self) -> u8 {
        self.depth
    }
    pub fn colour(&self) -> ColourType {
        self.colour
    }
    pub fn data(&mut self) -> &mut [u8] {
        self.data
    }
    pub fn set_ncol(mut self, ncol: usize) -> Self {
        self.ncol = ncol;
        self
    }
    pub fn set_nrow(mut self, nrow: usize) -> Self {
        self.nrow = nrow;
        self
    }
    pub fn set_depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }
    pub fn set_colour(mut self, colour: ColourType) -> Self {
        self.colour = colour;
        self
    }
    pub fn set_data(mut self, data: &'a mut [u8]) -> Self {
        self.data = data;
        self
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
