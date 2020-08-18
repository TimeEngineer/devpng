pub mod chunk;
pub mod crc;
pub mod datastream;

pub mod prelude {
    pub use crate::chunk::chunk::{Chunk, ChunkMut};
}