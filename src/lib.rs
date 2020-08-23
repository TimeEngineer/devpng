//! # Example
//!
//! ## Want to modify an image ?
//! ```
//! use devpng::prelude::PNG;
//!
//! fn main() -> Result<(), String> {
//!     // Load.
//!     let mut buf = std::fs::read("img.png")
//!         .expect("Couldn't read the file.");
//!     let mut png = PNG::load(&mut buf)?;
//!
//!     // Access Image.
//!     let img = png.image();
//!
//!     // Modify image.
//!     for i in 0..img.nrow * img.ncol {
//!         img.data[i] = !img.data[i];
//!     }
//!
//!     // Store.
//!     png.store("img.png")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Want to create an image ?
//! ```
//! use devpng::prelude::{ColourType, Image, PNG};
//!
//! fn main() -> Result<(), String> {
//!     let mut data = [255, 0, 0, 0, 0, 0];
//!     let img = Image::new(&mut data[..])
//!         .set_ncol(6)
//!         .set_nrow(1)
//!         .set_depth(8)
//!         .set_colour(ColourType::RGB);
//!     let mut buf = Vec::new();
//!     let mut png = PNG::from_image(&mut buf, &img);
//!
//!     // Store.
//!     png.store("img.png")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Want low level access ?
//! ```
//! use devpng::prelude::DataStreamMut;
//!
//! fn main() -> Result<(), String> {
//!     // Load.
//!     let mut buf = std::fs::read("img.png")
//!         .expect("Couldn't read the file.");
//!     let mut datastream = DataStreamMut::from(&mut buf)?;
//!
//!     // Access Image.
//!     let mut cache = datastream.idat()?;
//!     let img = cache.image();
//!    
//!     // Modify image.
//!     for i in 0..img.nrow * img.ncol {
//!         img.data[i] = !img.data[i];
//!     }
//!
//!     // Store.
//!     let png = datastream.rebuild(&mut Some(&mut cache));
//!     std::fs::write("img.png", png)
//!         .expect("Couldn't write the file.");
//!     Ok(())
//! }
//! ```

// Private.
mod chunk;
mod crc;

// Public.
pub mod cache;
pub mod colour;
pub mod datastream;
pub mod image;
pub mod png;

pub mod prelude {
    // Private.
    pub(crate) use crate::chunk::chunk::{Chunk, ChunkMut};
    // Public.
    pub use crate::cache::FiltCache;
    pub use crate::colour::ColourType;
    pub use crate::datastream::DataStreamMut;
    pub use crate::image::Image;
    pub use crate::png::PNG;
}
