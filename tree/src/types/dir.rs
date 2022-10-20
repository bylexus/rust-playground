use crate::types::File;
use std::vec::Vec;
use std::{io::Result, path::Path};

use super::Config;

#[derive(Debug)]
pub struct Dir {
    total_size: u64,
    // name: String,
    path: String,
    child_dirs: Vec<Dir>,
    child_files: Vec<File>,
}

impl Dir {
    pub fn examine(path_entry: &Path) -> Result<Dir> {
        if !path_entry.is_dir() {
            panic!("Only dirs allowed");
        }

        let mut dir = Dir {
            total_size: 0,
            // name: String::from(path_entry.file_name().unwrap().to_str().unwrap()),
            path: String::from(path_entry.as_os_str().to_str().unwrap()),
            child_dirs: Vec::new(),
            child_files: Vec::new(),
        };

        // examine tree:
        let rd = match std::fs::read_dir(path_entry) {
            Ok(dir_reader) => dir_reader,
            Err(e) => return Err(e),
        };

        for res in rd {
            let entry = match res {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Skipping: {}", err);
                    continue;
                }
            };

            let path_buf = entry.path();
            let dir_type = entry.file_type().unwrap();

            if dir_type.is_dir() {
                Dir::examine_dir(&mut dir, path_buf.as_path());
            } else if dir_type.is_file() {
                Dir::examine_file(&mut dir, path_buf.as_path());
            } else if dir_type.is_symlink() {
                continue;
            } else {
                eprintln!(
                    "Skipping {}, no regular file",
                    entry.file_name().to_str().unwrap_or("(unknown)")
                );
                continue;
            }
        }

        Ok(dir)
    }

    fn examine_dir(dir: &mut Dir, path: &Path) {
        let child_dir = match Dir::examine(path) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Skipping {:?}: {}", path, e);
                return;
            }
        };
        dir.total_size = dir.total_size + child_dir.size();
        dir.child_dirs.push(child_dir);
    }

    fn examine_file(dir: &mut Dir, path: &Path) {
        let child_file = match File::examine(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Skipping {:?}: {}", path, e);
                return;
            }
        };
        dir.total_size = dir.total_size + child_file.size();
        dir.child_files.push(child_file);
    }

    pub fn size(&self) -> u64 {
        self.total_size
    }

    pub fn print(&self, config: &Config) {
        if !config.summary {
            for dir in &self.child_dirs {
                dir.print(config);
            }
        }
        if !config.summary && config.print_files {
            for file in &self.child_files {
                file.print();
            }
        }
        println!("{}\t{}", self.size(), self.path);
    }
}
