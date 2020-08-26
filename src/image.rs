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
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, u8> {
        self.data.iter_mut()
    }
    pub fn chunks(&self, chunk_size: usize) -> std::slice::Chunks<u8> {
        self.data.chunks(chunk_size)
    }
    pub fn chunks_mut(&mut self, chunk_size: usize) -> std::slice::ChunksMut<u8> {
        self.data.chunks_mut(chunk_size)
    }
    pub fn chunks_exact(&self, chunk_size: usize) -> std::slice::ChunksExact<u8> {
        self.data.chunks_exact(chunk_size)
    }
    pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> std::slice::ChunksExactMut<u8> {
        self.data.chunks_exact_mut(chunk_size)
    }
    pub fn rchunks(&self, chunk_size: usize) -> std::slice::RChunks<u8> {
        self.data.rchunks(chunk_size)
    }
    pub fn rchunks_mut(&mut self, chunk_size: usize) -> std::slice::RChunksMut<u8> {
        self.data.rchunks_mut(chunk_size)
    }
    pub fn rchunks_exact(&self, chunk_size: usize) -> std::slice::RChunksExact<u8> {
        self.data.rchunks_exact(chunk_size)
    }
    pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> std::slice::RChunksExactMut<u8> {
        self.data.rchunks_exact_mut(chunk_size)
    }
    pub fn rows(&self) -> std::slice::Chunks<u8> {
        self.data.chunks(self.ncol)
    }
    pub fn rows_mut(&mut self) -> std::slice::ChunksMut<u8> {
        self.data.chunks_mut(self.ncol)
    }
    pub fn windows(&self, size: usize) -> std::slice::Windows<u8> {
        self.data.windows(size)
    }
}
impl<'a> std::fmt::Debug for Image<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("size: {}x{}\n", self.nrow, self.ncol));
        s.push_str(&format!("depth: {}\n", self.depth));
        s.push_str(&format!("colour: {:?}\n", self.colour));
        for row in self.rows() {
            s.push_str(&format!("{:02x?}\n", row));
        }
        write!(f, "{}", s)
    }
}
impl<'a> std::ops::Index<usize> for Image<'a> {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.ncol..(index + 1) * self.ncol]
    }
}
impl<'a> std::ops::IndexMut<usize> for Image<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.ncol..(index + 1) * self.ncol]
    }
}
