//! # Example
//!
//! ```
//! use devpng::prelude::DataStreamMut;
//!
//! fn main() -> Result<(), String> {
//!     // Load.
//!     let mut buf = std::fs::read("img0.png").expect("Couldn't read the file.");
//!     let mut datastream = DataStreamMut::from(&mut buf)?;
//!
//!     // Access Image.
//!     let mut idat = datastream.idat()?;
//!     let img = idat.image();
//!
//!     // Modify image.
//!     for i in 0..img.nrow * img.ncol {
//!         img.data[i] = !img.data[i];
//!     }
//!
//!     // Store.
//!     let png = datastream.rebuild(Some(idat));
//!     std::fs::write("img1.png", png).expect("Couldn't write the file.");
//!
//!     Ok(())
//! }
//! ```

pub mod cache;
mod chunk;
pub mod colour;
mod crc;
pub mod datastream;
pub mod image;

pub mod prelude {
    pub(crate) use crate::chunk::chunk::{Chunk, ChunkMut};
    pub use crate::colour::ColourType;
    pub use crate::datastream::DataStreamMut;
    pub use crate::image::Image;
}
