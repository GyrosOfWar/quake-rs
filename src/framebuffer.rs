use std::fs::File;
use std::io::prelude::*;
use std::{io, f32};

use bezier::BezierCurve;
use util::{Color, Vec2, step};
use lmp::LmpImage;

const PALETTE_FILE_NAME: &'static str = "palette.lmp";

pub struct Palette {
    colors: [Color; 256]
}

impl Palette {
    pub fn new() -> io::Result<Palette>  {
        let mut buf = [Color::default(); 256];
        let mut reader = io::BufReader::new(try!(File::open(PALETTE_FILE_NAME)));
        let mut bytes = vec![];
        try!(reader.read_to_end(&mut bytes));

        for (i, b) in bytes.chunks(3).enumerate() {
            let (r, g, b) = (b[0], b[1], b[2]);
            buf[i] = Color::new(r, g, b);
        }

        Ok(Palette {
            colors: buf
        })
    }

    pub fn get(&self, c: u8) -> Color {
        self.colors[c as usize]
    }
}

pub struct Framebuffer {
    /// Buffer of byte values, indexes into the palette.
    /// Size is width * height, treated like a fixed-size array.
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    /// Buffer of colors as they will be rendered to the screen.
    /// Size is width * height * 4 (32 bpp), also treated like a fixed-size array.
    color_buffer: Vec<u8>,
    pub palette: Palette
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            pixels: vec![0; height * width],
            width: width as usize,
            height: height as usize,
            color_buffer: vec![0; height * width * 4],
            palette: Palette::new().unwrap()
        }
    }

    #[inline]
    fn index(&self, y: usize, x: usize) -> usize {
        x * self.width + y
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, color: u8) {
        let i = self.index(x, y);
        self.pixels[i] = color;
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.pixels[self.index(x, y)]
    }

    pub fn fill(&mut self, color: u8) {
        for v in &mut self.pixels {
            *v = color;
        }
    }

    pub fn width(&self) -> usize { self.width }

    pub fn height(&self) -> usize { self.height }

    /// Copies the values currently in the `pixels` array to the
    /// color buffer and translates them through the palette.
    pub fn swap_buffers(&mut self) {
        let mut i = 0;
        for px in &self.pixels {
            let color = self.palette.get(*px);
            self.color_buffer[i  ] = color.r;
            self.color_buffer[i+1] = color.g;
            self.color_buffer[i+2] = color.b;
            self.color_buffer[i+3] = 0;

            i += 4;
        }
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn color_buffer(&self) -> &[u8] {
        &self.color_buffer
    }

    /// DDA line drawing.
    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        let dx = x2 as f32 - x1 as f32;
        let dy = y2 as f32 - y1 as f32;
        let m = dy / dx;
        let mut y = y1 as f32;

        for x in x1..x2 {
            let iy = y.round() as usize;
            self.set(x, iy, color);
            y += m;
        }
    }

    /// Bresenham line drawing
    pub fn bre_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u8) {
        let dx = (x1 - x0) as i32;
        let dy = (y1 - y0) as i32;
        let mut d = 2 * dy - dx;

        self.set(x0, y0, color);
        let mut y = y0;

        if d > 0 {
            y += 1;
            d -= 2 * dx;
        }

        for x in x0+1..x1  {
            self.set(x, y, color);
            d = d + (2 * dy);
            if d > 0 {
                y += 1;
                d -= 2 * dx;
            }
        }
    }

    /// Barycentric triangle drawing
    pub fn triangle(&mut self, v1: Vec2, v2: Vec2, v3: Vec2, color: u8) {
        let max_x = f32::max(v1.x, f32::max(v2.x, v3.x));
        let min_x = f32::min(v1.x, f32::min(v2.x, v3.x));
        let max_y = f32::max(v1.y, f32::max(v2.y, v3.y));
        let min_y = f32::min(v1.y, f32::min(v2.y, v3.y));

        let v1 = Vec2::new(v2.x - v1.x, v2.y - v1.y);
        let v2 = Vec2::new(v3.x - v1.x, v3.y - v1.y);
        for x in step(min_x, max_x) {
            for y in step(min_y, max_y) {
                let q = Vec2::new(x - v1.x, y - v1.y);
                let s = q.dot(v2) / v1.dot(v2);
                let t = v1.dot(q) / v1.dot(v2);

                if (s >= 0.0) && (t >= 0.0) && (s + t <= 1.0) {
                    self.set(x.round() as usize, y.round() as usize, color);
                }
            }
        }
    }

    pub fn rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u8) {
        let xm = x + width + 1;
        let ym = y + height + 1;
        for h in y..ym {
            for w in x..xm {
                self.set(w, h, color);
            }
        }
    }

    pub fn bezier(&mut self, curve: BezierCurve, n: usize, color: u8) {
        for p in curve.approximate(n) {
            self.set(p.x as usize, p.y as usize, color);
        }
    }
    
    pub fn draw_pic(&mut self, x_pos: usize, y_pos: usize, image: &LmpImage) {
        // Not the fastst solution, probably
        for j in 0..image.height() {
            for i in 0..image.width() {
                let x = i as usize + x_pos;
                let y = j as usize + y_pos;
                self.set(x, y, image.get(i, j));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_framebuffer() {
        let h = 20;
        let w = 16;

        let mut fb = Framebuffer::new(w, h);
        for i in 0..w {
            for j in 0..h {
                fb.set(i, j, 20);
            }
        }

        assert_eq!(fb.get(w-1, h-1), 20);
    }

    #[test]
    fn to_bytes() {
        let w = 200;
        let sz = w * w * 4;
        let palette_index = 4;

        let mut fb = Framebuffer::new(w, w);
        fb.fill(palette_index);
        let p = fb.palette.get(palette_index);
        fb.swap_buffers();
        let bytes = fb.color_buffer();
        assert_eq!(bytes.len(), sz);
        for b in bytes.chunks(4) {
            assert_eq!(b[0], p.r);
            assert_eq!(b[1], p.g);
            assert_eq!(b[2], p.b);
        }
    }

    #[test]
    fn test_set() {
        let w = 20;
        let h = 16;
        let mut fb = Framebuffer::new(w, h);

        for y in 0..h {
            for x in 0..w {
                fb.set(x, y, 3);
            }
        }

        for p in fb.pixels() {
            assert_eq!(*p, 3);
        }
    }
}

#[cfg(test)]
mod bench {
    use test::Bencher;
    use super::*;
    const WIDTH: usize = 2560;
    const HEIGHT: usize = 1440;

    #[bench]
    fn bench_dda(b: &mut Bencher) {
        let mut fb = Framebuffer::new(WIDTH, HEIGHT);
        b.iter(|| {
            fb.line(0, 0, 799, 599, 12);
        });
    }

    #[bench]
    fn bench_bresenham(b: &mut Bencher) {
        let mut fb = Framebuffer::new(WIDTH, HEIGHT);
        b.iter(|| {
            fb.bre_line(0, 0, 799, 599, 12);
        });
    }
}
