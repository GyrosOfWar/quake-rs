use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::mem;

const PALETTE_FILE_NAME: &'static str = "palette.lmp";

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Color {
    pub r: u8, 
    pub g: u8, 
    pub b: u8, 
    _unused: u8
}

pub struct Palette {
    colors: [Color; 256]
}

impl Palette {
    pub fn new() -> io::Result<Palette>  {
        let mut buf = [Color::default(); 256];
        let reader = io::BufReader::new(try!(File::open(PALETTE_FILE_NAME)));
        let mut color = Color::default();
        let mut c = 0;
        
        for (i, byte) in reader.bytes().enumerate() {
            let byte = try!(byte);
            let p = i % 3;
            
            match p {
                0 => color.r = byte,
                1 => color.g = byte,
                2 => {
                    color.b = byte;
                    buf[c] = color;
                    color = Color::default();
                    c += 1;
                },
                _ => unreachable!()
            }
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
    pub fn set(&mut self, x: usize, y: usize, color: u8) {
        self.pixels[x * self.width + y] = color;
    }
    
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.pixels[x * self.width + y]
    }
    
    pub fn fill(&mut self, color: u8) {
        for p in self.pixels.iter_mut() {
            *p = color;
        }
    }
    
    pub fn to_color_buffer(&self) -> Vec<u8> {
        let mut colors: Vec<_> = self.pixels.iter().map(|c| self.palette.get(*c)).collect();
        let cap = colors.capacity();
        let len = colors.len();
        let ptr = colors.as_mut_ptr();
        unsafe {
            mem::forget(colors);
            let bytes: *mut u8 = mem::transmute(ptr);
            let new_len = len * 4;
            let new_cap = cap * 4;
            Vec::from_raw_parts(bytes, new_len, new_cap)
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
    fn to_color_buffer() {
        let w = 200;
        let sz = w * w * 4;
        let palette_index = 4;
        
        let mut fb = Framebuffer::new(w, w);
        fb.fill(palette_index);
        let p = fb.palette.get(palette_index);
        let bytes = fb.to_color_buffer();
        assert_eq!(bytes.len(), sz);
        for b in bytes.chunks(4) {
            assert_eq!(b[0], p.r);
            assert_eq!(b[1], p.g);
            assert_eq!(b[2], p.b);
        }
    }
}