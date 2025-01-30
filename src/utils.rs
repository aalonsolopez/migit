// Module utils - Contains some utility functions
use std::path::PathBuf;

pub fn path_creator(path: Option<&str>) -> PathBuf {
    let current_dir = std::env::current_dir()
        .expect("Failed to get current directory");
        
    match path {
        Some(p) => current_dir.join(p),
        None => current_dir
    }
}