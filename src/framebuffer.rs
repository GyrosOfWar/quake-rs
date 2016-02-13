use std::fs::File;
use std::io::prelude::*;
use std::{io, mem};
use rand::{thread_rng, Rng};

const PALETTE_FILE_NAME: &'static str = "palette.lmp";

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Color {
    pub r: u8, 
    pub g: u8, 
    pub b: u8, 
    _unused: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b, _unused: 0 }
    }
}

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
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    pub palette: Palette,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Framebuffer {
            pixels: vec![0; width * height],
            width: width as usize,
            height: height as usize,
            palette: Palette::new().unwrap()
        }
    }
    
    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        x * self.height + y
    }
    
    #[inline]
    pub fn set(&mut self, x: usize, y: usize, color: u8) {
        let i = self.index(x, y);
        //println!("x={}, y={}", x, y);
        self.pixels[i] = color;
    }
    
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.pixels[self.index(x, y)]
    }
    
    pub fn fill(&mut self, color: u8) {
        for p in &mut self.pixels {
            *p = color;
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        // Look up the color indices in the palette.
        let mut colors: Vec<_> = self.pixels
            .iter()
            .map(|c| self.palette.get(*c))
            .collect();
        // A color is 4 bytes in size
        let new_cap = colors.capacity() * 4;
        let new_len = colors.len() * 4;
        let ptr = colors.as_mut_ptr();
        unsafe {
            mem::forget(colors);
            // ptr is a *mut Color
            let bytes: *mut u8 = mem::transmute(ptr);
            // Rebuild the vector with the new length and capacity.
            Vec::from_raw_parts(bytes, new_len, new_cap)
        }
    }
    
    /// DDA line drawing.
    pub fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: u8) {
        let dx = (x2 - x1) as f32;
        let dy = (y2 - y1) as f32;
        let steps = if dx > dy {
            dx.abs()
        } else {
            dy.abs()
        };
        
        let xin = dx / steps;
        let yin = dy / steps;
        
        let mut x: f32 = x1 as f32;
        let mut y: f32 = y1 as f32;

        for _ in 0..(steps as usize) {
            let xc = x.round() as usize;
            let yc = y.round() as usize;
            
            self.set(xc, yc, color);
            x += xin;
            y += yin;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_framebuffer() {
        let h = 400;
        let w = 300;
        
        let mut fb = Framebuffer::new(w, h);
        for i in 0..w {
            for j in 0..h {
                fb.set(i, j, 20);
            }
        }
        
        assert_eq!(fb.get(20, 20), 20);
    }
    
    #[test]
    fn to_bytes() {
        let w = 200;
        let sz = w * w * 4;
        let palette_index = 4;
        
        let mut fb = Framebuffer::new(w, w);
        fb.fill(palette_index);
        let p = fb.palette.get(palette_index);
        let bytes = fb.to_bytes();
        assert_eq!(bytes.len(), sz);
        for b in bytes.chunks(4) {
            assert_eq!(b[0], p.r);
            assert_eq!(b[1], p.g);
            assert_eq!(b[2], p.b);
        }
    }
    
    #[test]
    fn test_set() {
        let w = 400;
        let h = 300;
        let mut fb = Framebuffer::new(w, h);
        
        for y in 0..h {
            for x in 0..w {
                fb.set(x, y, 3);
            }
        }
        
        
    }
}