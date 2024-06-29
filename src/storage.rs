use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom, Result};

pub struct Storage {
    file: std::fs::File,
}

impl Storage {
    pub fn new(path: &str) -> Self {
        let file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        Storage { file }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.file.seek(SeekFrom::End(0)).unwrap();
        self.file.write_all(data).unwrap();
    }

    pub fn read(&mut self, position: u64, size: usize) -> Result<Vec<u8>> {
        self.file.seek(SeekFrom::Start(position))?;
        let mut buffer = vec![0; size];
        let bytes_read = self.file.read(&mut buffer)?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }
}