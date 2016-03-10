use files::*;
use byteorder::*;
use std::io;

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
        let len = try!(rdr.read_i32::<LittleEndian>());
        let off = try!(rdr.read_i32::<LittleEndian>());

        Ok(Header {
            magic: b"PACK",
            directory_offset: off,
            directory_length: len
        })
    }
}

#[cfg(test)]
mod test {
    use files::*;
    use super::Header;

    #[test]
    fn read_header() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read("Id1/PAK0.PAK").unwrap();
        let header = Header::read(&mut file_mgr, 0).unwrap();
        assert_eq!(header.directory_length, 21632);
        assert_eq!(header.directory_offset, 18254423);
    }
}