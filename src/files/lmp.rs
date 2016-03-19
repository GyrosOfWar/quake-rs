use std::{io, fmt};
use std::io::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct LmpImage<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> LmpImage<'a> {
    pub fn from_bytes(data: &'a [u8]) -> io::Result<LmpImage<'a>> {
        let mut cursor = io::Cursor::new(data);
        let width = try!(cursor.read_u32::<LittleEndian>());
        let height = try!(cursor.read_u32::<LittleEndian>());
        let bytes = &cursor.get_ref()[8..];

        Ok(LmpImage {
            width: width,
            height: height,
            data: bytes,
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

impl<'a> fmt::Debug for LmpImage<'a> {
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
