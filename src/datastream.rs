//! # DataStream

// Imports.
use crate::cache::FiltCache;
use crate::chunk::{
    bkgd::{Bkgd, BkgdMut},
    chrm::{Chrm, ChrmMut},
    gama::{Gama, GamaMut},
    hist::{Hist, HistMut},
    iccp::{Iccp, IccpMut},
    idat::{Idat, IdatMut},
    iend::{Iend, IendMut},
    ihdr::{Ihdr, IhdrMut},
    itxt::{Itxt, ItxtMut},
    phys::{Phys, PhysMut},
    plte::{Plte, PlteMut},
    sbit::{Sbit, SbitMut},
    splt::{Splt, SpltMut},
    srgb::{Srgb, SrgbMut},
    text::{Text, TextMut},
    time::{Time, TimeMut},
    trns::{Trns, TrnsMut},
    ztxt::{Ztxt, ZtxtMut},
};
use crate::colour::ColourType;
use crate::crc::Crc;
use crate::prelude::{Chunk, ChunkMut};
use std::convert::TryFrom;
use std::convert::TryInto;

// Constants.
// PNG SIGNATURE.
const PNG_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

//
const IHDR: [u8; 4] = [b'I', b'H', b'D', b'R'];
const PLTE: [u8; 4] = [b'P', b'L', b'T', b'E'];
const IDAT: [u8; 4] = [b'I', b'D', b'A', b'T']; // allow multiple
const IEND: [u8; 4] = [b'I', b'E', b'N', b'D'];

// Before PLTE and IDAT.
const CHRM: [u8; 4] = [b'c', b'H', b'R', b'M'];
const GAMA: [u8; 4] = [b'g', b'A', b'M', b'A'];
const ICCP: [u8; 4] = [b'i', b'C', b'C', b'P']; // Or sRGB
const SBIT: [u8; 4] = [b's', b'B', b'I', b'T'];
const SRGB: [u8; 4] = [b's', b'R', b'G', b'B']; // Or iCCP

// After PLTE before IDAT.
const BKGD: [u8; 4] = [b'b', b'K', b'G', b'D'];
const HIST: [u8; 4] = [b'h', b'I', b'S', b'T'];
const TRNS: [u8; 4] = [b't', b'R', b'N', b'S'];

// Before IDAT.
const PHYS: [u8; 4] = [b'p', b'H', b'Y', b's'];
const SPLT: [u8; 4] = [b's', b'P', b'L', b'T']; // allow multiple

// None.
const TIME: [u8; 4] = [b't', b'I', b'M', b'E'];
const ITXT: [u8; 4] = [b'i', b'T', b'X', b't']; // allow multiple
const TEXT: [u8; 4] = [b't', b'E', b'X', b't']; // allow multiple
const ZTXT: [u8; 4] = [b'z', b'T', b'X', b't']; // allow multiple

// CHUNK LAYOUT
/* LENGTH
A four-byte unsigned integer giving the number of bytes in the chunk's data field. The length counts
only the data field, not itself, the chunk type, or the CRC. Zero is a valid length. Although encoders
and decoders should treat the length as unsigned, its value shall not exceed 2^31-1 bytes. */
/* CHUNK TYPE
A sequence of four bytes defining the chunk type. Each byte of a chunk type is restricted to the
decimal values 65 to 90 and 97 to 122. These correspond to the uppercase and lowercase ISO 646 letters
(A-Z and a-z) respectively for convenience in description and examination of PNG datastreams. Encoders
and decoders shall treat the chunk types as fixed binary values, not character strings. For example, it
would not be correct to represent the chunk type IDAT by the equivalents of those letters in the UCS 2
character set. */
/* CHUNK DATA
The data bytes appropriate to the chunk type, if any. This field can be of zero length.*/
/* CRC
A four-byte CRC (Cyclic Redundancy Code) calculated on the preceding bytes in the chunk, including the
chunk type field and chunk data fields, but not including the length field. The CRC can be used to check
for corruption of the data. The CRC is always present, even for chunks containing no data. */

/* Color type
Greyscale = 0
Truecolour = 2
Indexed-colour = 3
Greyscale with alpha = 4
Truecolour with alpha = 6
*/

const ERROR_PNGHEADER: &str = "PNG header is missing";
const ERROR_IHDRHEADER: &str = "IHDR header is missing";
const ERROR_TRNSHEADER: &str = "tNRS can't match with this colour type";
const ERROR_BUFFER: &str = "DevPNG: Buffer overflow";
const ERROR_UNKNOWN: &str = "Unknown chunk";
const ERROR_IHDR: &str = "IHDR length";
const ERROR_PLTE: &str = "PLTE length";
const ERROR_IEND: &str = "IEND length";
const ERROR_CHRM: &str = "cHRM length";
const ERROR_GAMA: &str = "gAMA length";
const ERROR_SBIT: &str = "sBIT length";
const ERROR_SRGB: &str = "sRGB length";
const ERROR_BKGD: &str = "bKGD length";
const ERROR_HIST: &str = "hIST length";
const ERROR_PHYS: &str = "pHYs length";
const ERROR_TIME: &str = "tIME length";
const ERROR_COLOUR: &str = "ColourType";
const ERROR_DEPTH: &str = "BitDepth";

// Structures.
pub struct DataStreamMut<'a> {
    pub ihdr: &'a mut [u8],
    pub plte: Option<&'a mut [u8]>,
    pub idat: Vec<&'a mut [u8]>,
    pub iend: Option<&'a mut [u8]>,

    pub chrm: Option<&'a mut [u8]>,
    pub gama: Option<&'a mut [u8]>,
    pub iccp: Option<&'a mut [u8]>,
    pub sbit: Option<&'a mut [u8]>,
    pub srgb: Option<&'a mut [u8]>,

    pub bkgd: Option<&'a mut [u8]>,
    pub hist: Option<&'a mut [u8]>,
    pub trns: Option<&'a mut [u8]>,

    pub phys: Option<&'a mut [u8]>,
    pub splt: Vec<&'a mut [u8]>,

    pub time: Option<&'a mut [u8]>,
    pub itxt: Vec<&'a mut [u8]>,
    pub text: Vec<&'a mut [u8]>,
    pub ztxt: Vec<&'a mut [u8]>,
}

// Implementations.
impl<'a> DataStreamMut<'a> {
    pub fn from(buf: &'a mut [u8]) -> Result<Self, String> {
        // PNG HEADER & IHDR (mandatory)
        let (mut datastream, mut buf) = Self::read_png_header(buf)?;
        while buf.len() >= 12 {
            buf = datastream.read_chunk(buf)?;
        }
        assert_eq!(buf.len(), 0);
        Ok(datastream)
    }
    fn read_png_header(buf: &'a mut [u8]) -> Result<(Self, &'a mut [u8]), String> {
        if buf.len() < 8 {
            return Err(ERROR_PNGHEADER.into());
        }
        let chunk = <[u8; 8]>::try_from(&buf[0..8]).unwrap();
        if chunk != PNG_HEADER {
            return Err(ERROR_PNGHEADER.into());
        }
        let buf = buf.split_at_mut(8).1;

        let length = u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
        if buf.len() < 12 + length {
            return Err(ERROR_BUFFER.into());
        }
        let chunk = <[u8; 4]>::try_from(&buf[4..8]).unwrap();
        if chunk != IHDR {
            return Err(ERROR_IHDRHEADER.into());
        }
        if length != 13 {
            return Err(ERROR_IHDR.into());
        }
        let (ihdr, buf) = buf.split_at_mut(12 + length);
        // Check BitDepth & ColourType.
        match ihdr[16] {
            1 | 2 | 4 | 8 | 16 => {}
            _ => return Err(ERROR_DEPTH.into()),
        }
        match ihdr[17] {
            0 | 2 | 3 | 4 | 6 => {}
            _ => return Err(ERROR_COLOUR.into()),
        }
        Ok((
            Self {
                ihdr,
                plte: None,
                idat: Vec::new(),
                iend: None,

                chrm: None,
                gama: None,
                iccp: None,
                sbit: None,
                srgb: None,

                bkgd: None,
                hist: None,
                trns: None,

                phys: None,
                splt: Vec::new(),

                time: None,
                itxt: Vec::new(),
                text: Vec::new(),
                ztxt: Vec::new(),
            },
            buf,
        ))
    }
    fn read_chunk(&mut self, buf: &'a mut [u8]) -> Result<&'a mut [u8], String> {
        let length = u32::from_be_bytes(buf[0..4].try_into().unwrap()) as usize;
        if buf.len() < 12 + length {
            return Err(ERROR_BUFFER.into());
        }
        let chunk = <[u8; 4]>::try_from(&buf[4..8]).unwrap();
        Ok(match chunk {
            PLTE => {
                if length % 3 != 0 {
                    return Err(ERROR_PLTE.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.plte = Some(chunk);
                left
            }
            IDAT => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.idat.push(chunk);
                left
            }
            IEND => {
                if length != 0 {
                    return Err(ERROR_IEND.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.iend = Some(chunk);
                left
            }
            CHRM => {
                if length != 32 {
                    return Err(ERROR_CHRM.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.chrm = Some(chunk);
                left
            }
            GAMA => {
                if length != 4 {
                    return Err(ERROR_GAMA.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.gama = Some(chunk);
                left
            }
            ICCP => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.iccp = Some(chunk);
                left
            }
            SBIT => {
                if length > 4 {
                    return Err(ERROR_SBIT.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.sbit = Some(chunk);
                left
            }
            SRGB => {
                if length != 1 {
                    return Err(ERROR_SRGB.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.srgb = Some(chunk);
                left
            }
            BKGD => {
                if length != 1 && length != 2 && length != 6 {
                    return Err(ERROR_BKGD.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.bkgd = Some(chunk);
                left
            }
            HIST => {
                if length % 2 != 0 {
                    return Err(ERROR_HIST.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.hist = Some(chunk);
                left
            }
            TRNS => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.trns = Some(chunk);
                left
            }
            PHYS => {
                if length != 9 {
                    return Err(ERROR_PHYS.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.phys = Some(chunk);
                left
            }
            SPLT => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.splt.push(chunk);
                left
            }
            TIME => {
                if length != 7 {
                    return Err(ERROR_TIME.into());
                }
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.time = Some(chunk);
                left
            }
            ITXT => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.itxt.push(chunk);
                left
            }
            TEXT => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.text.push(chunk);
                left
            }
            ZTXT => {
                let (chunk, left) = buf.split_at_mut(12 + length);
                self.ztxt.push(chunk);
                left
            }
            _ => {
                return Err(ERROR_UNKNOWN.into());
            }
        })
    }
    pub fn check_crc(&self) -> Result<(), String> {
        let ihdr = Ihdr::from(self.ihdr);
        let colour = ihdr.colour_type();
        ihdr.check_crc()?;
        if let Some(chunk) = &self.plte {
            Plte::from(*chunk).check_crc()?;
        }
        for chunk in &self.idat {
            Idat::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.iend {
            Iend::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.chrm {
            Chrm::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.gama {
            Gama::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.iccp {
            Iccp::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.sbit {
            Sbit::from(*chunk, colour).check_crc()?;
        }
        if let Some(chunk) = &self.srgb {
            Srgb::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.bkgd {
            Bkgd::from(*chunk, colour).check_crc()?;
        }
        if let Some(chunk) = &self.hist {
            Hist::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.trns {
            Trns::from(*chunk, colour).check_crc()?;
        }
        if let Some(chunk) = &self.phys {
            Phys::from(*chunk).check_crc()?;
        }
        for chunk in &self.splt {
            Splt::from(*chunk).check_crc()?;
        }
        if let Some(chunk) = &self.time {
            Time::from(*chunk).check_crc()?;
        }
        for chunk in &self.itxt {
            Itxt::from(*chunk).check_crc()?;
        }
        for chunk in &self.text {
            Text::from(*chunk).check_crc()?;
        }
        for chunk in &self.ztxt {
            Ztxt::from(*chunk).check_crc()?;
        }
        Ok(())
    }
    pub fn compute_crc(&mut self) -> Result<(), String> {
        let mut ihdr = IhdrMut::from(self.ihdr);
        let colour = ihdr.colour_type();
        ihdr.compute_crc();
        if let Some(chunk) = &mut self.plte {
            PlteMut::from(*chunk).compute_crc();
        }
        for chunk in &mut self.idat {
            IdatMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.iend {
            IendMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.chrm {
            ChrmMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.gama {
            GamaMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.iccp {
            IccpMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.sbit {
            SbitMut::from(*chunk, colour).compute_crc();
        }
        if let Some(chunk) = &mut self.srgb {
            SrgbMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.bkgd {
            BkgdMut::from(*chunk, colour).compute_crc();
        }
        if let Some(chunk) = &mut self.hist {
            HistMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.trns {
            TrnsMut::from(*chunk, colour).compute_crc();
        }
        if let Some(chunk) = &mut self.phys {
            PhysMut::from(*chunk).compute_crc();
        }
        for chunk in &mut self.splt {
            SpltMut::from(*chunk).compute_crc();
        }
        if let Some(chunk) = &mut self.time {
            TimeMut::from(*chunk).compute_crc();
        }
        for chunk in &mut self.itxt {
            ItxtMut::from(*chunk).compute_crc();
        }
        for chunk in &mut self.text {
            TextMut::from(*chunk).compute_crc();
        }
        for chunk in &mut self.ztxt {
            ZtxtMut::from(*chunk).compute_crc();
        }
        Ok(())
    }
    pub fn idat(&self) -> Result<FiltCache, String> {
        let ihdr = Ihdr::from(self.ihdr);
        let mut v = Vec::new();
        for idat in &self.idat {
            let idat = Idat::from(*idat);
            let data = idat.data();
            v.extend_from_slice(data);
        }
        FiltCache::build(
            ihdr.width(),
            ihdr.height(),
            ihdr.bit_depth(),
            ihdr.colour_type(),
            &v,
        )
    }
    pub fn rebuild(&mut self, idat: Option<FiltCache>) -> Vec<u8> {
        let mut encoded = match idat {
            Some(mut idat) => {
                let idat = idat.rebuild();
                let length = (idat.len() as u32).to_be_bytes();
                let chunk = IDAT;
                let mut crc = Crc::new();
                crc.update(&chunk);
                crc.update(&idat);
                [&length[..], &chunk[..], &idat, &crc.checksum()[..]].concat()
            }
            None => {
                let mut idat = Vec::new();
                for x in &self.idat {
                    let x = Idat::from(*x);
                    let data = x.data();
                    idat.extend_from_slice(data);
                }
                let length = (idat.len() as u32).to_be_bytes();
                let chunk = IDAT;
                let mut crc = Crc::new();
                crc.update(&chunk);
                crc.update(&idat);
                [&length[..], &chunk[..], &idat, &crc.checksum()[..]].concat()
            }
        };
        let mut v_out = Vec::new();
        v_out.extend(&PNG_HEADER);
        v_out.extend_from_slice(self.ihdr);
        // Before PLTE.
        if let Some(chunk) = &self.chrm {
            v_out.extend_from_slice(chunk);
        }
        if let Some(chunk) = &self.gama {
            v_out.extend_from_slice(chunk);
        }
        if let Some(chunk) = &self.iccp {
            v_out.extend_from_slice(chunk);
        }
        if let Some(chunk) = &self.sbit {
            v_out.extend_from_slice(chunk);
        }
        if let Some(chunk) = &self.srgb {
            v_out.extend_from_slice(chunk);
        }
        // PLTE.
        if let Some(chunk) = &self.plte {
            v_out.extend_from_slice(chunk);
        }
        // After PLTE.
        if let Some(chunk) = &self.bkgd {
            v_out.extend_from_slice(chunk);
        }
        if let Some(chunk) = &self.hist {
            v_out.extend_from_slice(chunk);
        }
        if let Some(chunk) = &self.trns {
            v_out.extend_from_slice(chunk);
        }
        // Before IDAT.
        if let Some(chunk) = &self.phys {
            v_out.extend_from_slice(chunk);
        }
        for chunk in &self.splt {
            v_out.extend_from_slice(chunk);
        }
        // IDAT.
        v_out.append(&mut encoded);
        // Others.
        if let Some(chunk) = &self.time {
            v_out.extend_from_slice(chunk);
        }
        for chunk in &self.itxt {
            v_out.extend_from_slice(chunk);
        }
        for chunk in &self.text {
            v_out.extend_from_slice(chunk);
        }
        for chunk in &self.ztxt {
            v_out.extend_from_slice(chunk);
        }
        // IEND.
        for chunk in &self.iend {
            v_out.extend_from_slice(chunk);
        }
        v_out
    }
}

impl<'a> std::fmt::Debug for DataStreamMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ihdr = Ihdr::from(self.ihdr);
        let colour = ihdr.colour_type();
        let mut s = format!("PNG Header\n");
        s.push_str(&format!("{:?}", ihdr));
        if let Some(chunk) = &self.plte {
            s.push_str(&format!("{:?}", Plte::from(*chunk)));
        }
        for chunk in &self.idat {
            s.push_str(&format!("{:?}", Idat::from(*chunk)));
        }
        if let Some(chunk) = &self.iend {
            s.push_str(&format!("{:?}", Iend::from(*chunk)));
        }
        if let Some(chunk) = &self.chrm {
            s.push_str(&format!("{:?}", Chrm::from(*chunk)));
        }
        if let Some(chunk) = &self.gama {
            s.push_str(&format!("{:?}", Gama::from(*chunk)));
        }
        if let Some(chunk) = &self.iccp {
            s.push_str(&format!("{:?}", Iccp::from(*chunk)));
        }
        if let Some(chunk) = &self.sbit {
            s.push_str(&format!("{:?}", Sbit::from(*chunk, colour)));
        }
        if let Some(chunk) = &self.srgb {
            s.push_str(&format!("{:?}", Srgb::from(*chunk)));
        }
        if let Some(chunk) = &self.bkgd {
            s.push_str(&format!("{:?}", Bkgd::from(*chunk, colour)));
        }
        if let Some(chunk) = &self.hist {
            s.push_str(&format!("{:?}", Hist::from(*chunk)));
        }
        if let Some(chunk) = &self.trns {
            s.push_str(&format!(
                "{:?}",
                Trns::from(
                    *chunk,
                    match colour {
                        ColourType::GreyscaleAlpha | ColourType::TruecolourAlpha =>
                            return write!(f, "{}", ERROR_TRNSHEADER),
                        colour => colour,
                    }
                )
            ));
        }
        if let Some(chunk) = &self.phys {
            s.push_str(&format!("{:?}", Phys::from(*chunk)));
        }
        for chunk in &self.splt {
            s.push_str(&format!("{:?}", Splt::from(*chunk)));
        }
        if let Some(chunk) = &self.time {
            s.push_str(&format!("{:?}", Time::from(*chunk)));
        }
        for chunk in &self.itxt {
            s.push_str(&format!("{:?}", Itxt::from(*chunk)));
        }
        for chunk in &self.text {
            s.push_str(&format!("{:?}", Text::from(*chunk)));
        }
        for chunk in &self.ztxt {
            s.push_str(&format!("{:?}", Ztxt::from(*chunk)));
        }
        write!(f, "{}", s)
    }
}
