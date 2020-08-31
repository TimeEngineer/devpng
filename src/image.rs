//! # Image

// Imports.
use crate::prelude::ColourType;
use std::cmp::Ordering;
// Structures.
pub struct Image<'a> {
    pub ncol: usize,
    pub nrow: usize,
    pub depth: u8,
    pub colour: ColourType,
    pub data: &'a mut [u8],
}
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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
    fn convert_identity(&mut self, img: &Image) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let min = std::cmp::min(row0.len(), row1.len());
            (&mut row0[..min]).copy_from_slice(&row1[..min])
        }
    }
    // assert!(n < m).
    fn convert_n_from_m(&mut self, img: &Image, n: usize, m: usize) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(n).zip((0..len1).step_by(m)) {
                (&mut row0[i0..i0 + n]).copy_from_slice(&row1[i1..i1 + n]);
            }
        }
    }
    // assert!(n < m).
    fn convert_m_from_n(&mut self, img: &Image, m: usize, n: usize) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(m).zip((0..len1).step_by(n)) {
                (&mut row0[i0..i0 + n]).copy_from_slice(&row1[i1..i1 + n]);
                for left in &mut row0[i0 + n..i0 + m] {
                    *left = 255;
                }
            }
        }
    }
    fn convert_g8_from_rgb8(&mut self, img: &Image) {
        let step = match img.colour {
            ColourType::RGB => 3,
            ColourType::RGBA => 4,
            _ => unreachable!(),
        };
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).zip((0..len1).step_by(step)) {
                let sum = (row1[i1] as u16) + (row1[i1 + 1] as u16) + (row1[i1 + 2] as u16);
                row0[i0] = (sum / 3) as u8;
            }
        }
    }
    fn convert_ga8_from_rgb8(&mut self, img: &Image) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(2).zip((0..len1).step_by(3)) {
                let sum = (row1[i1] as u16) + (row1[i1 + 1] as u16) + (row1[i1 + 2] as u16);
                row0[i0] = (sum / 3) as u8;
                row0[i0 + 1] = 255;
            }
        }
    }
    fn convert_ga8_from_rgba8(&mut self, img: &Image) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(2).zip((0..len1).step_by(4)) {
                let sum = (row1[i1] as u16) + (row1[i1 + 1] as u16) + (row1[i1 + 2] as u16);
                row0[i0] = (sum / 3) as u8;
                row0[i0 + 1] = row1[i1 + 3];
            }
        }
    }
    fn convert_rgb8_from_g8(&mut self, img: &Image) {
        let step = match img.colour {
            ColourType::Greyscale => 1,
            ColourType::GreyscaleAlpha => 2,
            _ => unreachable!(),
        };
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(3).zip((0..len1).step_by(step)) {
                let pix = row1[i1];
                row0[i0] = pix;
                row0[i0 + 1] = pix;
                row0[i0 + 2] = pix;
            }
        }
    }
    fn convert_rgba8_from_g8(&mut self, img: &Image) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(4).zip(0..len1) {
                let pix = row1[i1];
                row0[i0] = pix;
                row0[i0 + 1] = pix;
                row0[i0 + 2] = pix;
                row0[i0 + 3] = 255;
            }
        }
    }
    fn convert_rgba8_from_ga8(&mut self, img: &Image) {
        for (row0, row1) in self.rows_mut().zip(img.rows()) {
            let len0 = row0.len();
            let len1 = row1.len();
            let row0 = &mut row0[..len0];
            let row1 = &row1[..len1];
            for (i0, i1) in (0..len0).step_by(4).zip((0..len1).step_by(2)) {
                let pix = row1[i1];
                row0[i0] = pix;
                row0[i0 + 1] = pix;
                row0[i0 + 2] = pix;
                row0[i0 + 3] = row1[i1 + 1];
            }
        }
    }
    pub fn from_image(mut self, img: &Image) -> Self {
        match (self.depth, img.depth, self.colour, img.colour) {
            // Identity.
            (1, 1, ColourType::Greyscale, ColourType::Greyscale)
            | (2, 2, ColourType::Greyscale, ColourType::Greyscale)
            | (4, 4, ColourType::Greyscale, ColourType::Greyscale)
            | (8, 8, ColourType::Greyscale, ColourType::Greyscale)
            | (16, 16, ColourType::Greyscale, ColourType::Greyscale)
            | (8, 8, ColourType::RGB, ColourType::RGB)
            | (16, 16, ColourType::RGB, ColourType::RGB)
            | (1, 1, ColourType::Indexed, ColourType::Indexed)
            | (2, 2, ColourType::Indexed, ColourType::Indexed)
            | (4, 4, ColourType::Indexed, ColourType::Indexed)
            | (8, 8, ColourType::Indexed, ColourType::Indexed)
            | (8, 8, ColourType::GreyscaleAlpha, ColourType::GreyscaleAlpha)
            | (16, 16, ColourType::GreyscaleAlpha, ColourType::GreyscaleAlpha)
            | (8, 8, ColourType::RGBA, ColourType::RGBA)
            | (16, 16, ColourType::RGBA, ColourType::RGBA) => {
                self.convert_identity(img);
            }
            // BitDepth 8 | 8
            (8, 8, ColourType::Greyscale, ColourType::RGB)
            | (8, 8, ColourType::Greyscale, ColourType::RGBA) => {
                self.convert_g8_from_rgb8(img);
            }
            (8, 8, ColourType::Greyscale, ColourType::GreyscaleAlpha) => {
                self.convert_n_from_m(img, 1, 2);
            }
            (8, 8, ColourType::RGB, ColourType::Greyscale)
            | (8, 8, ColourType::RGB, ColourType::GreyscaleAlpha) => {
                self.convert_rgb8_from_g8(img);
            }
            (8, 8, ColourType::RGB, ColourType::RGBA) => {
                self.convert_n_from_m(img, 3, 4);
            }
            (8, 8, ColourType::GreyscaleAlpha, ColourType::Greyscale) => {
                self.convert_m_from_n(img, 2, 1);
            }
            (8, 8, ColourType::GreyscaleAlpha, ColourType::RGB) => {
                self.convert_ga8_from_rgb8(img);
            }
            (8, 8, ColourType::GreyscaleAlpha, ColourType::RGBA) => {
                self.convert_ga8_from_rgba8(img);
            }
            (8, 8, ColourType::RGBA, ColourType::Greyscale) => {
                self.convert_rgba8_from_g8(img);
            }
            (8, 8, ColourType::RGBA, ColourType::RGB) => {
                self.convert_m_from_n(img, 4, 3);
            }
            (8, 8, ColourType::RGBA, ColourType::GreyscaleAlpha) => {
                self.convert_rgba8_from_ga8(img);
            }
            // BitDepth 16 | 16

            // BitDepth 8 | 16
            (8, 16, ColourType::Greyscale, ColourType::Greyscale)
            | (8, 16, ColourType::RGB, ColourType::RGB)
            | (8, 16, ColourType::GreyscaleAlpha, ColourType::GreyscaleAlpha)
            | (8, 16, ColourType::RGBA, ColourType::RGBA) => {
                self.convert_n_from_m(img, 1, 2);
            }
            // BitDepth 16 | 8
            (16, 8, ColourType::Greyscale, ColourType::Greyscale)
            | (16, 8, ColourType::RGB, ColourType::RGB)
            | (16, 8, ColourType::GreyscaleAlpha, ColourType::GreyscaleAlpha)
            | (16, 8, ColourType::RGBA, ColourType::RGBA) => {
                self.convert_m_from_n(img, 2, 1);
            }
            _ => unimplemented!(),
        }
        self
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
    pub fn plot(&mut self, pt: Point, colour: &[u8]) {
        let bpp = colour.len();
        let x = pt.x as usize;
        let y = pt.y as usize;
        (&mut self[y][x * bpp..(x + 1) * bpp]).copy_from_slice(colour);
    }
    pub fn plot_line(&mut self, pt0: Point, pt1: Point, colour: &[u8]) {
        let (x0, y0, x1, y1) = (pt0.x, pt0.y, pt1.x, pt1.y);
        let (mut x, mut y, xf, yf, dx, sx) = match (x0.cmp(&x1), y0.cmp(&y1)) {
            (Ordering::Equal, Ordering::Equal) => {
                self.plot(pt0, colour);
                return;
            }
            (Ordering::Less, Ordering::Equal) => {
                let bpp = colour.len();
                let x = x0 as usize;
                let y = y0 as usize;
                let dx = (x1 - x0) as usize;
                for pix in self[y].chunks_mut(bpp).skip(x).take(dx) {
                    pix.copy_from_slice(colour);
                }
                return;
            }
            (Ordering::Greater, Ordering::Equal) => {
                let bpp = colour.len();
                let x = x1 as usize;
                let y = y0 as usize;
                let dx = (x0 - x1) as usize;
                for pix in self[y].chunks_mut(bpp).skip(x).take(dx) {
                    pix.copy_from_slice(colour);
                }
                return;
            }
            (Ordering::Equal, Ordering::Less) => {
                let bpp = colour.len();
                let x = x0 as usize;
                let y = y0 as usize;
                let dy = (y1 - y0) as usize;
                for row in self.rows_mut().skip(y).take(dy) {
                    let pix = &mut row[x * bpp..(x + 1) * bpp];
                    pix.copy_from_slice(colour);
                }
                return;
            }
            (Ordering::Equal, Ordering::Greater) => {
                let bpp = colour.len();
                let x = x0 as usize;
                let y = y1 as usize;
                let dy = (y0 - y1) as usize;
                for row in self.rows_mut().skip(y).take(dy) {
                    let pix = &mut row[x * bpp..(x + 1) * bpp];
                    pix.copy_from_slice(colour);
                }
                return;
            }
            (Ordering::Less, Ordering::Less) => (x0, y0, x1, y1, x1 - x0, 1),
            (Ordering::Greater, Ordering::Less) => (x0, y0, x1, y1, x0 - x1, -1),
            (Ordering::Less, Ordering::Greater) => (x1, y1, x0, y0, x1 - x0, 1),
            (Ordering::Greater, Ordering::Greater) => (x1, y1, x0, y0, x0 - x1, -1),
        };
        let dy = y0 - y1;
        let mut err = dx + dy;
        loop {
            self.plot(Point { x, y }, colour);
            if x == xf && y == yf {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += 1;
            }
        }
    }
    pub fn plot_triangle(&mut self, pt0: Point, pt1: Point, pt2: Point, colour: &[u8]) {
        self.plot_line(pt0, pt1, colour);
        self.plot_line(pt0, pt2, colour);
        self.plot_line(pt1, pt2, colour);
    }
    pub fn plot_quadrilateral(
        &mut self,
        pt0: Point,
        pt1: Point,
        pt2: Point,
        pt3: Point,
        colour: &[u8],
    ) {
        self.plot_line(pt0, pt1, colour);
        self.plot_line(pt1, pt2, colour);
        self.plot_line(pt2, pt3, colour);
        self.plot_line(pt3, pt0, colour);
    }
    pub fn plot_pentagon(
        &mut self,
        pt0: Point,
        pt1: Point,
        pt2: Point,
        pt3: Point,
        pt4: Point,
        colour: &[u8],
    ) {
        self.plot_line(pt0, pt1, colour);
        self.plot_line(pt1, pt2, colour);
        self.plot_line(pt2, pt3, colour);
        self.plot_line(pt3, pt4, colour);
        self.plot_line(pt4, pt1, colour);
    }
    pub fn plot_hexagon(
        &mut self,
        pt0: Point,
        pt1: Point,
        pt2: Point,
        pt3: Point,
        pt4: Point,
        pt5: Point,
        colour: &[u8],
    ) {
        self.plot_line(pt0, pt1, colour);
        self.plot_line(pt1, pt2, colour);
        self.plot_line(pt2, pt3, colour);
        self.plot_line(pt3, pt4, colour);
        self.plot_line(pt4, pt5, colour);
        self.plot_line(pt5, pt1, colour);
    }
    pub fn plot_parallelogram(&mut self, pt0: Point, pt1: Point, pt2: Point, colour: &[u8]) {
        let pt3 = pt2 - pt0 + pt1;
        self.plot_line(pt0, pt1, colour);
        self.plot_line(pt0, pt2, colour);
        self.plot_line(pt1, pt3, colour);
        self.plot_line(pt2, pt3, colour);
    }
    pub fn plot_circle(&mut self, center: Point, r: i32, colour: &[u8]) {
        let x0 = center.x;
        let y0 = center.y;
        let mut x = r;
        let mut y = 0;
        let mut m = 5 - 4 * r;
        while y <= x {
            self.plot(Point::new(x0 - y, y0 - x), colour);
            self.plot(Point::new(x0 + y, y0 - x), colour);
            self.plot(Point::new(x0 - x, y0 - y), colour);
            self.plot(Point::new(x0 + x, y0 - y), colour);
            self.plot(Point::new(x0 - x, y0 + y), colour);
            self.plot(Point::new(x0 + x, y0 + y), colour);
            self.plot(Point::new(x0 - y, y0 + x), colour);
            self.plot(Point::new(x0 + y, y0 + x), colour);
            if m > 0 {
                x -= 1;
                m -= 8 * x;
            }
            y += 1;
            m += 8 * y + 4;
        }
    }
    pub fn plot_circle_andres(&mut self, center: Point, r: i32, colour: &[u8]) {
        let x0 = center.x;
        let y0 = center.y;
        let mut x = r;
        let mut y = 0;
        let mut d = r - 1;
        while y <= x {
            self.plot(Point::new(x0 - y, y0 - x), colour);
            self.plot(Point::new(x0 + y, y0 - x), colour);
            self.plot(Point::new(x0 - x, y0 - y), colour);
            self.plot(Point::new(x0 + x, y0 - y), colour);
            self.plot(Point::new(x0 - x, y0 + y), colour);
            self.plot(Point::new(x0 + x, y0 + y), colour);
            self.plot(Point::new(x0 - y, y0 + x), colour);
            self.plot(Point::new(x0 + y, y0 + x), colour);
            if d >= 2 * y {
                d -= 2 * y - 1;
                y += 1;
            } else if d < 2 * (r - x) {
                d += 2 * x - 1;
                x -= 1;
            } else {
                d += 2 * (x - y - 1);
                x -= 1;
                y += 1;
            }
        }
    }
    pub fn plot_disk(&mut self, center: Point, r: i32, colour: &[u8]) {
        let x0 = center.x;
        let y0 = center.y;
        let mut x = r;
        let mut y = 0;
        let mut m = 5 - 4 * r;
        while y <= x {
            self.plot_line(
                Point::new(x0 - y, y0 - x),
                Point::new(x0 + y, y0 - x),
                colour,
            );
            self.plot_line(
                Point::new(x0 - x, y0 - y),
                Point::new(x0 + x, y0 - y),
                colour,
            );
            self.plot_line(
                Point::new(x0 - x, y0 + y),
                Point::new(x0 + x, y0 + y),
                colour,
            );
            self.plot_line(
                Point::new(x0 - y, y0 + x),
                Point::new(x0 + y, y0 + x),
                colour,
            );
            if m > 0 {
                x -= 1;
                m -= 8 * x;
            }
            y += 1;
            m += 8 * y + 4;
        }
    }
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
