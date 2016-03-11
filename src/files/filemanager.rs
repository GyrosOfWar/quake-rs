use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

pub type FileHandle = usize;

#[derive(Debug, Default)]
pub struct FileManager {
    open_files: Vec<File>,
    filenames: Vec<PathBuf>,
}

impl FileManager {
    pub fn new() -> FileManager {
        FileManager {
            open_files: vec![],
            filenames: vec![],
        }
    }

    pub fn open_read<P>(&mut self, path: P) -> io::Result<FileHandle>
        where P: AsRef<Path>
    {
        let pb = path.as_ref().to_path_buf();
        let file = try!(File::open(path));
        self.open_files.push(file);
        self.filenames.push(pb);

        Ok(self.open_files.len() - 1)
    }

    pub fn open_write<P>(&mut self, path: P) -> io::Result<FileHandle>
        where P: AsRef<Path>
    {
        // TODO try out ? operator
        let file = try!(File::create(path));
        self.open_files.push(file);
        Ok(self.open_files.len() - 1)
    }

    pub fn close(&mut self, handle: FileHandle) {
        // Since we're deleting the element from the vector, the element drops
        // out of scope in this function, which also closes the file handle.
        // RAII is great.
        self.open_files.remove(handle);
        self.filenames.remove(handle);
    }

    pub fn seek(&mut self, handle: FileHandle, pos: io::SeekFrom) -> io::Result<u64> {
        self.open_files[handle].seek(pos)
    }

    pub fn read(&mut self, handle: FileHandle, buffer: &mut [u8]) -> io::Result<usize> {
        let mut file = &self.open_files[handle];
        file.read(buffer)
    }

    pub fn read_to_end(&mut self, handle: FileHandle, buffer: &mut Vec<u8>) -> io::Result<usize> {
        self.open_files[handle].read_to_end(buffer)
    }

    pub fn write(&mut self, handle: FileHandle, source: &[u8]) -> io::Result<usize> {
        self.open_files[handle].write(source)
    }

    pub fn write_all(&mut self, handle: FileHandle, source: &[u8]) -> io::Result<()> {
        self.open_files[handle].write_all(source)
    }

    pub fn close_all(&mut self) {
        self.open_files.clear();
        self.filenames.clear();
    }

    pub fn filename(&self, handle: FileHandle) -> Option<&str> {
        // let path = &self.filenames[handle];
        // path.file_name().unwrap().to_str().unwrap()

        self.filenames.get(handle).and_then(|f| f.file_name().and_then(|g| g.to_str()))
    }
}
