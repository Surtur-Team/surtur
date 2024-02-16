pub mod error;
/// Provides various utility functions
pub mod macros;

use std::{fs, io::empty, path::PathBuf};

pub const MISSING_CFG: &str = "Failed to find the project's config file (project.lua)";

pub fn root_dir_name(cur_dir: &str) -> &str {
    let dirs: Vec<&str> = cur_dir.split('/').collect();
    dirs.last().unwrap_or_else(|| {
        panic!(
            "Failed to get current dir. Provided dir: {} is invalid",
            cur_dir
        )
    })
}

pub fn create_dir(dir: &str) {
    match fs::create_dir(dir) {
        Ok(_) => (),
        Err(err) => panic!("Failed to create dir: {}", err),
    }
}

pub fn get_src_files(path: &PathBuf) -> Vec<PathBuf> {
    let dir = fs::read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to find directory: {}", path.display()));
    dir.flatten()
        .map(|entry| {
            let file_type = entry.file_type().expect("Failed to get file type");
            if file_type.is_dir() {
                get_src_files(&entry.path())
            } else {
                let file_name = entry.file_name().to_string_lossy().to_string();
                let file_ending = &file_name[file_name.len() - 2..];
                if file_ending == ".c" {
                    vec![entry.path()]
                } else {
                    vec![]
                }
            }
        })
        .flatten()
        .collect()
}
