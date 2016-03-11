use files::*;
use byteorder::{ReadBytesExt, LittleEndian};
use std::{io, fmt, str};
use std::io::prelude::*;

#[derive(Debug)]
pub enum PackError {
    IoError(io::Error),
    UnknownContentFileName,
    UnknownPakFileName,
}

impl From<io::Error> for PackError {
    fn from(err: io::Error) -> PackError {
        PackError::IoError(err)
    }
}

pub type PackResult<T> = Result<T, PackError>;

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
        let nul = name_bytes.iter().position(|b| *b == 0).unwrap();
        let valid = &name_bytes[..nul];
        str::from_utf8(valid).unwrap()
    }
}

impl fmt::Debug for PackFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackFile {{ name: {}, position: {}, length: {} }}", self.name_str(), self.position, self.length)
    }
}

#[derive(Debug)]
pub struct Pack {
    name: String,
    files: Vec<PackFile>,
    handle: FileHandle,
}

impl Pack {
    pub fn open(file_mgr: &mut FileManager, handle: FileHandle, name: String) -> io::Result<Pack> {
        let header = try!(Header::read(file_mgr, handle));
        try!(file_mgr.seek(handle, io::SeekFrom::Start(header.directory_offset as u64)));
        let file_count = header.directory_length as usize / PackFile::size();
        let mut files = vec![];
        
        for _ in 0..file_count {
            let file = try!(PackFile::read(file_mgr, handle));
            files.push(file);
        }
        
        Ok(Pack {
            name: name,
            files: files,
            handle: handle,
        })
    }
    
    pub fn read_file(&self, name: &str, file_mgr: &mut FileManager) -> PackResult<Vec<u8>> {
        let file = self.files.iter().find(|f| f.name_str() == name);
        match file {
            Some(f) => {
                try!(file_mgr.seek(self.handle, io::SeekFrom::Start(f.position as u64)));
                let mut buf = vec![0; f.length as usize];
                let mut bytes_read = 0;
                while bytes_read < f.length {
                    bytes_read += try!(file_mgr.read(self.handle, &mut buf)) as i32;
                }
                Ok(buf)
            },
            None => Err(PackError::UnknownContentFileName)
        }
    }
}

pub struct PackContainer {
    files: Vec<Pack>,
    file_mgr: FileManager,
}

impl PackContainer {
    pub fn new() -> PackContainer {
        PackContainer {
            files: vec![],
            file_mgr: FileManager::new()
        }
    }
    
    pub fn file_mgr(&mut self) -> &mut FileManager {
        &mut self.file_mgr
    }
    
    pub fn get(&self, idx: usize) -> &Pack {
        &self.files[idx]
    }
    
    pub fn read(&mut self, pak_filename: &str, filename: &str) -> PackResult<Vec<u8>> {
        let pak = self.files.iter().find(|f| f.name == pak_filename);
        match pak {
            Some(p) => {
                p.read_file(filename, &mut self.file_mgr)
            },
            None => return Err(PackError::UnknownPakFileName)
        }
    }
    
    pub fn read_pack(&mut self, path: &str) -> PackResult<()> {
        let handle = try!(self.file_mgr.open_read(path));
        let name = try!(self.file_mgr.filename(handle).ok_or(PackError::UnknownContentFileName)).into();
        let pack = try!(Pack::open(&mut self.file_mgr, handle, name));
        self.files.push(pack);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use files::{FileManager, FileHandle};
    use super::{PackContainer, Header, Pack, PackFile, PackResult};

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
    
    #[test]
    fn read_whole_pack() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read("Id1/PAK0.PAK").unwrap();
        
        let pak0 = Pack::open(&mut file_mgr, 0, "PAK0.PAK".into()).unwrap();
        assert_eq!(pak0.files.len(), 339);
    }
    
    #[test]
    fn read_file_from_pack() {
        let mut file_mgr = FileManager::new();
        let h = file_mgr.open_read("Id1/PAK0.PAK").unwrap();
        
        let mut pak0 = Pack::open(&mut file_mgr, h, "PAK0.PAK".into()).unwrap();
        let file = pak0.read_file("gfx/palette.lmp", &mut file_mgr).unwrap();
        assert_eq!(file[0], 0);
        assert_eq!(file[1], 0);
        assert_eq!(file[2], 0);
    }
}