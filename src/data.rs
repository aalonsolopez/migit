// Module data - Responsible for handling the data in .migit directory

use std::fs;
use sha1::{Digest, Sha1};

const MIGIT_DIR: &str = ".migit";

pub fn init_directory() -> i8 {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => panic!("Failed to get current directory: {}", e)
    };
    match fs::create_dir(MIGIT_DIR) {
        Ok(_) => {
            println!("Initialized empty migit repository in {}/{}", current_dir, MIGIT_DIR);
            0
        },
        Err(e) => {
            println!("There has been an error {}", e);
            1
        }
    }
}

pub fn hash_object(data: Vec<u8>) -> i8 {
    let mut hasher = Sha1::new();
    hasher.update(&data);

    let hashed_data = hasher.finalize();

    let formated_hash = format!("{:?}", hashed_data);

    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => panic!("Failed to get current directory: {}", e)
    };
    
    let path_to_write: String = format!("{}/{}/{}", current_dir, MIGIT_DIR, formated_hash);
    
    let file_write_result = fs::write(path_to_write, &data);

    match file_write_result {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    }
}