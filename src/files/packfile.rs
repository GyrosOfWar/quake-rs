use files::*;
use byteorder::{ReadBytesExt, LittleEndian};
use std::{io, fmt, str};
use std::path::Path;
use std::io::prelude::*;
use util;

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

/// Header of the PAK file format. Contains 4 bytes ("PACK") identifying the
/// file, the offset of the directory contents and the length of the directory.
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
            directory_length: len,
        })
    }

    pub fn size() -> usize {
        12
    }
}

/// A directory entry in the PAK file, identifying a single content file
/// in the PAK. Contains a name (56 bytes), an offset from the start of the file
/// (4 bytes) and the size of the file (4 bytes).
struct PackFile {
    name: [u8; 56],
    position: i32,
    length: i32,
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
            length: length,
        })
    }

    pub fn size() -> usize {
        56 + 4 + 4
    }

    pub fn name_str(&self) -> &str {
        let name_bytes = &self.name;
        let nul = name_bytes.iter().position(|b| *b == 0).unwrap();
        let valid = &name_bytes[..nul];
        str::from_utf8(valid).unwrap()
    }
}

impl fmt::Debug for PackFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "PackFile {{ name: {}, position: {}, length: {} }}",
               self.name_str(),
               self.position,
               self.length)
    }
}

/// A PAK file. Has a name, a list of directory entries and an associated file handle.
#[derive(Debug)]
pub struct Pack {
    name: String,
    files: Vec<PackFile>,
    handle: FileHandle,
}

impl Pack {
    /// Opens a PAK file for reading with the supplied FileManager and file handle.
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

    /// Reads the contents of a file within this PAK and returns it as a `Vec<u8>`.
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
            }
            None => Err(PackError::UnknownContentFileName),
        }
    }
}

/// A structure containing one or more PAK files and a file manager, for convenience.
#[derive(Default, Debug)]
pub struct PackContainer {
    files: Vec<Pack>,
    file_mgr: FileManager,
}

impl PackContainer {
    pub fn new() -> PackContainer {
        PackContainer {
            files: vec![],
            file_mgr: FileManager::new(),
        }
    }

    pub fn file_mgr(&mut self) -> &mut FileManager {
        &mut self.file_mgr
    }

    pub fn get(&self, idx: usize) -> &Pack {
        &self.files[idx]
    }

    /// Looks through the PAK files and tries to find the given file and reads it into a buffer.
    pub fn read(&mut self, filename: &str) -> PackResult<Vec<u8>> {
        for pak in &self.files {
            let result = pak.read_file(filename, &mut self.file_mgr);
            match result {
                r@Ok(_) => return r,
                Err(_) => continue,
            }
        }
        Err(PackError::UnknownContentFileName)
    }

    /// Opens a PAK file, reads the contents of its directory and appends the contents to the list
    /// of PAK files of this PackContainer.
    pub fn read_pack<P>(&mut self, path: P) -> PackResult<()>
        where P: AsRef<Path>
    {
        let handle = try!(self.file_mgr.open_read(path));
        let name = try!(self.file_mgr.filename(handle).ok_or(PackError::UnknownContentFileName))
                       .into();
        let pack = try!(Pack::open(&mut self.file_mgr, handle, name));
        self.files.push(pack);
        Ok(())
    }

    pub fn add_game_directory<P>(&mut self, path: P) -> PackResult<()>
        where P: AsRef<Path>
    {
        // Some arbitrary number, I'm not sure where Quake starts counting
        const HIGHEST_PAK_NUMBER: isize = 16;

        let mut i = HIGHEST_PAK_NUMBER;
        while i >= 0 {
            let filename = path.as_ref().join(format!("PAK{}.PAK", i));
            util::ignore(self.read_pack(filename));
            i -= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use files::FileManager;
    use super::{Header, Pack, PackFile, PackContainer};

    const PAK0: &'static str = "Id1/PAK0.PAK";

    #[test]
    fn read_header() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read(PAK0).unwrap();
        let header = Header::read(&mut file_mgr, 0).unwrap();
        assert_eq!(header.directory_length, 21696);
        assert_eq!(header.directory_offset, 18254423);
    }

    #[test]
    fn read_packfile() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read(PAK0).unwrap();
        let header = Header::read(&mut file_mgr, 0).unwrap();
        file_mgr.seek(0, io::SeekFrom::Start(header.directory_offset as u64)).unwrap();
        let packfile = PackFile::read(&mut file_mgr, 0).unwrap();
        assert_eq!(packfile.name_str(), "sound/items/r_item1.wav");
    }

    #[test]
    fn read_whole_pack() {
        let mut file_mgr = FileManager::new();
        file_mgr.open_read(PAK0).unwrap();

        let pak0 = Pack::open(&mut file_mgr, 0, "PAK0.PAK".into()).unwrap();
        assert_eq!(pak0.files.len(), 339);
    }

    #[test]
    fn read_file_from_pack() {
        let mut file_mgr = FileManager::new();
        let h = file_mgr.open_read(PAK0).unwrap();

        let pak0 = Pack::open(&mut file_mgr, h, "PAK0.PAK".into()).unwrap();
        let file = pak0.read_file("gfx/palette.lmp", &mut file_mgr).unwrap();
        assert_eq!(file[0], 0);
        assert_eq!(file[1], 0);
        assert_eq!(file[2], 0);
        assert_eq!(file[3], 15);
        assert_eq!(file[4], 15);
        assert_eq!(file[5], 15);
    }

    #[test]
    fn pack_container() {
        let mut pc = PackContainer::new();
        pc.read_pack("Id1/PAK0.PAK").unwrap();
        let file = pc.read("PAK0.PAK", "gfx/palette.lmp").unwrap();
        assert_eq!(file[0], 0);
        assert_eq!(file[1], 0);
        assert_eq!(file[2], 0);
        assert_eq!(file[3], 15);
        assert_eq!(file[4], 15);
        assert_eq!(file[5], 15);

    }
}
