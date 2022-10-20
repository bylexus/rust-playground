use std::{io::Result, path::Path};

#[derive(Debug)]
pub struct File {
    size: u64,
    // name: String,
    path: String,
}

impl File {
    pub fn examine(path_entry: &Path) -> Result<File> {
        let meta = match std::fs::metadata(path_entry) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };

        let f = File {
            size: meta.len(),
            // name: String::from(path_entry.file_name().unwrap().to_str().unwrap()),
            path: String::from(path_entry.to_str().unwrap()),
        };

        Ok(f)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn print(&self) {
        println!("{}\t{}", self.size(), self.path);
    }
}
