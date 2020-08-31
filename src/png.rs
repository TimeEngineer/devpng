//! # PNG
//!
//! High Level Interface.

// Imports.
use crate::datastream::{IEND_CHUNK, PNG_HEADER};
use crate::prelude::{DataStreamMut, FiltCache, Image};
// Structures.
pub struct PNG<'a> {
    pub datastream: DataStreamMut<'a>,
    pub cache: FiltCache,
}
// Implementations.
impl<'a> PNG<'a> {
    pub fn load(buf: &'a mut [u8]) -> Result<Self, String> {
        let datastream = DataStreamMut::from(buf)?;
        let cache = datastream.idat()?;
        Ok(Self { datastream, cache })
    }
    pub fn store<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), String> {
        let png = self.datastream.rebuild(&mut Some(&mut self.cache));
        match std::fs::write(path, png) {
            Ok(_) => Ok(()),
            Err(e) => return Err(format!("{}", e)),
        }
    }
    pub fn from_image(buf: &'a mut Vec<u8>, img: &Image) -> Self {
        let mut cache = FiltCache::from_image(img);
        let mut ihdr = DataStreamMut::build_ihdr_with_cache(&mut cache);
        let mut idat = DataStreamMut::build_idat_with_cache(&mut cache);
        buf.clear();
        buf.extend_from_slice(&PNG_HEADER);
        buf.append(&mut ihdr);
        buf.append(&mut idat);
        buf.extend_from_slice(&IEND_CHUNK);
        let datastream = DataStreamMut::from(&mut buf[..]).unwrap();
        Self { datastream, cache }
    }
    pub fn image(&mut self) -> Image {
        self.cache.image()
    }
    pub fn filter(&mut self) -> Image {
        self.cache.filter()
    }
    pub fn remove_filter(&mut self) {
        self.cache.remove_filter();
    }
    pub fn compute_crc(&mut self) {
        self.datastream.compute_crc();
    }
    pub fn check_crc(&self) -> Result<(), String> {
        self.datastream.check_crc()
    }
}

impl<'a> std::fmt::Debug for PNG<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.datastream)
    }
}
impl<'a> std::fmt::Display for PNG<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.datastream)
    }
}
