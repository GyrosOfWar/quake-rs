use files::*;
use byteorder::*;
use std::{io, fmt, str};
use std::io::prelude::*;

#[derive(Debug)]
struct Header {
    magic: &'static [u8],
    directory_offset: i32,
    directory_length: i32,
}

impl Header {
    pub fn read(file_mgr: &mut FileManager, handle: FileHandle) -> io::Result<Header> {
        let mut buf = [0; 12];
        try!(file_mgr.read(handle, &mut buf));
        let mut rdr = io::Cursor::new(buf);
        let mut magic = [0; 4];
        try!(rdr.read_exact(&mut magic));
        assert_eq!(&magic, b"PACK");
        let off = try!(rdr.read_i32::<LittleEndian>());
        let len = try!(rdr.read_i32::<LittleEndian>());

        Ok(Header {
            magic: b"PACK",
            directory_offset: off,
            directory_length: len
        })
    }
    
    pub fn size() -> usize { 12 }
}

struct PackFile {
    name: [u8; 56],
    position: i32,
    length: i32
}

impl PackFile {
    pub fn read(file_mgr: &mut FileManager, handle: FileHandle) -> io::Result<PackFile> {
        let mut buffer = vec![0; 64];
        try!(file_mgr.read(handle, &mut buffer));
        let mut rdr = io::Cursor::new(buffer);
        let mut name = [0; 56];
        try!(rdr.read_exact(&mut name));
        
        let pos = try!(rdr.read_i32::<LittleEndian>());
        let length = try!(rdr.read_i32::<LittleEndian>());
        
        Ok(PackFile {
            name: name,
            position: pos,
            length: length
        })
    }
    
    pub fn size() -> usize { 56 + 4 + 4 }
    
    pub fn name_str(&self) -> &str {
        let name_bytes = &self.name;
        let first_nul_idx = name_bytes.iter().position(|b| *b == 0).unwrap();
        &str::from_utf8(name_bytes).unwrap()[..first_nul_idx]
    }
}

impl fmt::Debug for PackFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackFile {{ name: {}, position: {}, length: {} }}", self.name_str(), self.position, self.length)
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use files::*;
    use super::{Header, PackFile};

    #[test]
    fn read_header() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read("Id1/PAK0.PAK").unwrap();
        let header = Header::read(&mut file_mgr, 0).unwrap();
        assert_eq!(header.directory_length, 21696);
        assert_eq!(header.directory_offset, 18254423);
    }
    
    #[test]
    fn read_packfile() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read("Id1/PAK0.PAK").unwrap();
        let header = Header::read(&mut file_mgr, 0).unwrap();
        file_mgr.seek(0, io::SeekFrom::Start(header.directory_offset as u64)).unwrap();
        let packfile = PackFile::read(&mut file_mgr, 0).unwrap();
        assert_eq!(packfile.name_str(), "sound/items/r_item1.wav");
    }
}