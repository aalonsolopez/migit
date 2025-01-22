// Module data - Responsible for handling the data in .migit directory

use std::fs;

const MIGIT_DIR: &str = ".migit";

pub fn init_directory() {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => panic!("Failed to get current directory: {}", e),
        
    };
    fs::create_dir(MIGIT_DIR).expect("Failed to create .migit directory");
    println!("Initialized empty migit repository in {}/{}", current_dir, MIGIT_DIR);
}