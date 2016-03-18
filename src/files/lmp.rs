use std::{io, fmt};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct LmpImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl LmpImage {
    pub fn from_file<P>(path: P) -> io::Result<LmpImage>
        where P: AsRef<Path>
    {
        let file = try!(File::open(path));
        let mut reader = io::BufReader::new(file);
        let width = try!(reader.read_u32::<LittleEndian>());
        let height = try!(reader.read_u32::<LittleEndian>());
        let mut data = vec![];
        try!(reader.read_to_end(&mut data));
        Ok(LmpImage {
            width: width,
            height: height,
            data: data,
        })
    }

    #[inline]
    fn index(&self, y: u32, x: u32) -> usize {
        (x * self.width + y) as usize
    }

    pub fn get(&self, x: u32, y: u32) -> u8 {
        self.data[self.index(x, y)]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[u8] {
        &self.data
    }
}

impl fmt::Debug for LmpImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!("{} ", self.data[self.index(x, y)]));
            }

            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
