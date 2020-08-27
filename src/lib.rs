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
//!     // Access image.
//!     let img = png.image();
//!
//!     // Modify image.
//!     for x in img.iter_mut() {
//!         *x = !*x;
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
//! use devpng::prelude::{ColourType, Image, PNG, Point};
//! 
//! fn main() -> Result<(), String> {
//!     let mut data = vec![255; 800 * 200];
//!     let img = Image::new(&mut data[..])
//!         .set_ncol(800) // 200
//!         .set_nrow(200)
//!         .set_depth(8)
//!         .set_colour(ColourType::RGBA);
//!     let mut buf = Vec::new();
//! 
//!     let mut png = PNG::from_image(&mut buf, &img);
//!     let mut img = png.image();
//! 
//!     for i in 0..50 {
//!         let center = Point { x: 100, y: 100 };
//!         let radius = 80 - i as i32;
//!         let colour = &[0, (255 - i * 5) as u8, (255 - i * 5) as u8, 255];
//!         img.plot_circle(center, radius, colour);
//!     }
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
//!     // Access image.
//!     let mut cache = datastream.idat()?;
//!     let img = cache.image();
//!    
//!     // Modify image.
//!     for x in img.iter_mut() {
//!         *x = !*x;
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
    pub use crate::image::{Image, Point};
    pub use crate::png::PNG;
}
