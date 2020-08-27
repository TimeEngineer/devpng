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
