// Module data - Responsible for handling the data in .migit directory

use std::fs;
use sha1::{Digest, Sha1};

use crate::utils;

const MIGIT_DIR: &str = ".migit";

pub fn init_directory() -> i8 {
    let current_dir = utils::path_creator(None);
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

    let formated_hash = format!("{:x}", hashed_data);

    let current_dir = utils::path_creator(None);
    
    let path_to_write: String = format!("{}/{}/{}", current_dir, MIGIT_DIR, formated_hash);

    match fs::write(path_to_write, &data) {
        Ok(_) => {
            println!("File created! Object {} created in {}", formated_hash, MIGIT_DIR);
            0
        },
        Err(e) => {
            println!("Error writing file: {}", e);
            1
        }
    }
}