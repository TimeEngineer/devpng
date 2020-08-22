//! # ColourType

// Structures.
#[derive(Debug, Clone, Copy)]
pub enum ColourType {
    Greyscale = 0,
    Truecolour = 2,
    Indexed = 3,
    GreyscaleAlpha = 4,
    TruecolourAlpha = 6,
}
// Implementations.
impl ColourType {
    pub fn from(colourtype: u8) -> Self {
        match colourtype {
            0 => Self::Greyscale,
            2 => Self::Truecolour,
            3 => Self::Indexed,
            4 => Self::GreyscaleAlpha,
            6 => Self::TruecolourAlpha,
            _ => unreachable!(),
        }
    }
}
