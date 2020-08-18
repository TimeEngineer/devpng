//! # Chunk

// Imports.
use crate::crc::Crc;
use std::convert::TryInto;

// Constants.
const ERROR_CRC: &str = "Crc check failed";

// Traits.
pub trait Chunk {
    fn inner(&self) -> &[u8];
    fn data(&self) -> &[u8] {
        let inner = self.inner();
        let length = u32::from_be_bytes(inner[0..4].try_into().unwrap()) as usize;
        &inner[8..8 + length]
    }
    fn check_crc(&self) -> Result<(), String> {
        let mut crc = Crc::new();
        let inner = self.inner();
        let length = u32::from_be_bytes(inner[0..4].try_into().unwrap()) as usize;
        let data = &inner[4..8 + length];
        crc.update(data);
        let crc = crc.checksum();
        let _crc: [u8; 4] = inner[8 + length..].try_into().unwrap();
        if crc != _crc {
            let chunk = std::str::from_utf8(&inner[4..8]).unwrap();
            return Err(format!(
                "{}: {} your: 0x{:08x} correct: 0x{:08x}",
                ERROR_CRC,
                chunk,
                u32::from_be_bytes(_crc),
                u32::from_be_bytes(crc)
            ));
        }
        Ok(())
    }
}
pub trait ChunkMut {
    fn inner(&mut self) -> &mut [u8];
    fn compute_crc(&mut self) {
        let mut crc = Crc::new();
        let inner = self.inner();
        let length = u32::from_be_bytes(inner[0..4].try_into().unwrap()) as usize;
        let data = &inner[4..8 + length];
        crc.update(data);
        (&mut inner[8 + length..]).copy_from_slice(&crc.checksum());
    }
}