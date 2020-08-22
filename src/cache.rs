//! # FiltCache

// Imports.
use crate::colour::ColourType;
use crate::image::Image;
use devker::prelude::{zlib_decode_to, zlib_encode, BlockType, Cache};
// Structures.
pub struct FiltCache {
    pub width: usize,
    pub height: usize,
    pub bit_depth: u8,
    pub colour_type: ColourType,
    pub bpp: usize,
    pub data: Vec<u8>,
    pub cache: Cache,
}
// Implementations.
impl FiltCache {
    pub fn build(
        width: u32,
        height: u32,
        bit_depth: u8,
        colour_type: ColourType,
        idat: &[u8],
    ) -> Result<Self, String> {
        let npix = match colour_type {
            ColourType::Greyscale | ColourType::Indexed => 1,
            ColourType::Truecolour => 3,
            ColourType::GreyscaleAlpha => 2,
            ColourType::TruecolourAlpha => 4,
        };
        let width = match bit_depth {
            1 => (width as usize) >> 3 + if width % 8 == 0 { 0 } else { 1 },
            2 => (width as usize) >> 2 + if width % 4 == 0 { 0 } else { 1 },
            4 => (width as usize) >> 1 + if width % 2 == 0 { 0 } else { 1 },
            8 => width as usize,
            16 => (width as usize) << 1,
            _ => unreachable!(),
        } * npix;
        let height = height as usize;

        let bpp = match bit_depth {
            1 | 2 | 4 | 8 => 1,
            16 => 2,
            _ => unreachable!(),
        } * npix;

        let mut cache = Cache::new();
        let mut data = vec![0; height * (2 * width + 1)];
        let (_, filt) = data.split_at_mut(height * width);
        zlib_decode_to(idat, &mut cache, filt)?;

        Ok(Self {
            width,
            height,
            bit_depth,
            colour_type,
            bpp,
            data,
            cache,
        })
    }
    pub fn image(&mut self) -> Image {
        self.recon();
        let (orig, _) = self.data.split_at_mut(self.height * self.width);
        Image::from(
            self.width,
            self.height,
            self.bit_depth,
            self.colour_type,
            orig,
        )
    }
    pub fn filter(&mut self) -> Image {
        let (_, filt) = self.data.split_at_mut(self.height * self.width);
        Image::from(
            self.width + 1,
            self.height,
            self.bit_depth,
            self.colour_type,
            filt,
        )
    }
    pub fn rebuild(&mut self) -> Vec<u8> {
        self.filt();
        let (_, filt) = self.data.split_at(self.height * self.width);
        zlib_encode(filt, BlockType::Fixed, &mut self.cache)
    }
    pub fn remove_filter(&mut self) {
        let (_, filt) = self.data.split_at_mut(self.height * self.width);
        for i in 0..self.height {
            filt[i * (self.width + 1)] = 0;
        }
    }
    fn recon(&mut self) {
        let (orig, filt) = self.data.split_at_mut(self.height * self.width);
        for i in 0..self.height {
            match filt[i * (self.width + 1)] {
                0 => nofilt(
                    &filt[i * (self.width + 1) + 1..(i + 1) * (self.width + 1)],
                    &mut orig[i * self.width..(i + 1) * self.width],
                ),
                1 => recon1(
                    &filt[i * (self.width + 1) + 1..(i + 1) * (self.width + 1)],
                    &mut orig[i * self.width..],
                    self.bpp,
                ),
                2 => {
                    let (prev, orig) = orig.split_at_mut(i * self.width);
                    recon2(
                        &filt[i * (self.width + 1) + 1..(i + 1) * (self.width + 1)],
                        &prev[(i - 1) * self.width..],
                        orig,
                    );
                }
                3 => {
                    let (prev, orig) = orig.split_at_mut(i * self.width);
                    recon3(
                        &filt[i * (self.width + 1) + 1..(i + 1) * (self.width + 1)],
                        &prev[(i - 1) * self.width..],
                        orig,
                        self.bpp,
                    );
                }
                4 => {
                    let (prev, orig) = orig.split_at_mut(i * self.width);
                    recon4(
                        &filt[i * (self.width + 1) + 1..(i + 1) * (self.width + 1)],
                        &prev[(i - 1) * self.width..],
                        orig,
                        self.bpp,
                    );
                }
                _ => unimplemented!(),
            }
        }
    }
    fn filt(&mut self) {
        let (orig, filt) = self.data.split_at_mut(self.height * self.width);
        for i in 0..self.height {
            match filt[i * (self.width + 1)] {
                0 => nofilt(
                    &orig[i * self.width..(i + 1) * self.width],
                    &mut filt[i * (self.width + 1) + 1..(i + 1) * (self.width + 1)],
                ),
                1 => filt1(
                    &orig[i * self.width..(i + 1) * self.width],
                    &mut filt[i * (self.width + 1) + 1..],
                    self.bpp,
                ),
                2 => filt2(
                    &orig[i * self.width..(i + 1) * self.width],
                    &orig[(i - 1) * self.width..],
                    &mut filt[i * (self.width + 1) + 1..],
                ),
                3 => filt3(
                    &orig[i * self.width..(i + 1) * self.width],
                    &orig[(i - 1) * self.width..],
                    &mut filt[i * (self.width + 1) + 1..],
                    self.bpp,
                ),
                4 => filt4(
                    &orig[i * self.width..(i + 1) * self.width],
                    &orig[(i - 1) * self.width..],
                    &mut filt[i * (self.width + 1) + 1..],
                    self.bpp,
                ),
                _ => unimplemented!(),
            }
        }
    }
}

fn nofilt(v_in: &[u8], v_out: &mut [u8]) {
    v_out.copy_from_slice(v_in);
}

fn recon1(v_in: &[u8], v_out: &mut [u8], bpp: usize) {
    let r = v_in.len();
    let mut a = 0;
    let mut x;
    for j in 0..bpp {
        x = v_in[j];
        v_out[j] = x.wrapping_add(a);
    }
    for j in bpp..r {
        a = v_out[j - bpp];
        x = v_in[j];
        v_out[j] = x.wrapping_add(a);
    }
}

fn recon2(v_in: &[u8], prev: &[u8], v_out: &mut [u8]) {
    let r = v_in.len();
    let mut b;
    let mut x;
    for j in 0..r {
        x = v_in[j];
        b = prev[j];
        v_out[j] = x.wrapping_add(b);
    }
}

fn recon3(v_in: &[u8], prev: &[u8], v_out: &mut [u8], bpp: usize) {
    let r = v_in.len();
    let mut a = 0;
    let mut b;
    let mut x;
    for j in 0..bpp {
        b = prev[j];
        x = v_in[j];
        v_out[j] = x.wrapping_add((a + b) >> 1);
    }
    for j in bpp..r {
        a = v_out[j - bpp];
        b = prev[j];
        x = v_in[j];
        v_out[j] = x.wrapping_add(((a as u16 + b as u16) >> 1) as u8);
    }
}

fn recon4(v_in: &[u8], prev: &[u8], v_out: &mut [u8], bpp: usize) {
    let r = v_in.len();
    let mut a = 0;
    let mut c = 0;
    let mut b;
    let mut x;
    let mut pr;
    for j in 0..bpp {
        b = prev[j];
        x = v_in[j];
        pr = b;
        v_out[j] = x.wrapping_add(pr);
    }
    for j in (bpp..r) {
        a = v_out[j - bpp];
        b = prev[j];
        c = prev[j - bpp];
        x = v_in[j];
        pr = paeth_predictor(a, b, c);
        v_out[j] = x.wrapping_add(pr);
    }
}

fn filt1(v_in: &[u8], v_out: &mut [u8], bpp: usize) {
    let r = v_in.len();
    let mut a = 0;
    let mut x;
    for j in 0..bpp {
        x = v_in[j];
        v_out[j] = x.wrapping_sub(a);
    }
    for j in bpp..r {
        a = v_in[j - bpp];
        x = v_in[j];
        v_out[j] = x.wrapping_sub(a);
    }
}

fn filt2(v_in: &[u8], prev: &[u8], v_out: &mut [u8]) {
    let r = v_in.len();
    let mut a;
    let mut x;
    for j in 0..r {
        a = prev[j];
        x = v_in[j];
        v_out[j] = x.wrapping_sub(a);
    }
}

fn filt3(v_in: &[u8], prev: &[u8], v_out: &mut [u8], bpp: usize) {
    let r = v_in.len();
    let mut a = 0;
    let mut b;
    let mut x;
    for j in 0..bpp {
        b = prev[j];
        x = v_in[j];
        v_out[j] = x.wrapping_sub((a + b) >> 1);
    }
    for j in bpp..r {
        a = v_in[j - bpp];
        b = prev[j];
        x = v_in[j];
        v_out[j] = x.wrapping_sub(((a as u16 + b as u16) >> 1) as u8);
    }
}

fn filt4(v_in: &[u8], prev: &[u8], v_out: &mut [u8], bpp: usize) {
    let r = v_in.len();
    let mut a = 0;
    let mut b;
    let mut c = 0;
    let mut x;
    let mut pr;
    for j in 0..bpp {
        b = prev[j];
        x = v_in[j];
        pr = b;
        v_out[j] = x.wrapping_sub(pr);
    }
    for j in (bpp..r) {
        a = v_in[j - bpp];
        b = prev[j];
        c = prev[j - bpp];
        x = v_in[j];
        pr = paeth_predictor(a, b, c);
        v_out[j] = x.wrapping_sub(pr);
    }
}

fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let p = a as i16 + b as i16 - c as i16;
    let pa = (p - a as i16).abs();
    let pb = (p - b as i16).abs();
    let pc = (p - c as i16).abs();
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}
