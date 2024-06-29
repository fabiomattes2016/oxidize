use crate::storage::Storage;
use std::io::Result;

pub struct Database {
    storage: Storage,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Database {
            storage: Storage::new(path),
        }
    }

    pub fn insert(&mut self, data: &str) {
        self.storage.write(data.as_bytes());
    }

    pub fn get(&mut self, position: u64, size: usize) -> Result<String> {
        let data = self.storage.read(position, size)?;
        Ok(String::from_utf8(data).unwrap())
    }
}