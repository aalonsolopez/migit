// Module data - Responsible for handling the data in .migit directory

use std::fs;
use sha1::{Digest, Sha1};

use crate::utils;

pub fn init_directory() -> i8 {
    let current_dir = utils::path_creator(None);
    let mut error_flag: i8 = 0;
    
    match fs::create_dir(crate::MIGIT_DIR) {
        Ok(_) => {
            println!("Created .migit directory");
        },
        Err(e) => {
            println!("There has been an error creating .migit directory: {}", e);
            error_flag = 1;
        }
    }

    if error_flag != 1 {
        match fs::create_dir(format!("{}/objects", crate::MIGIT_DIR)) {
            Ok(_) => {
                println!("Created .migit/objects directory");
            },
            Err(e) => {
                println!("There has been an error creating .migit/object directory: {}", e);
                error_flag = 1;
            }
        }
    }

    println!("Initialized empty migit repository in {}/{}", current_dir, crate::MIGIT_DIR);
    
    error_flag
}

pub fn hash_object(data: Vec<u8>) -> i8 {
    let mut hasher = Sha1::new();
    hasher.update(&data);

    let hashed_data = hasher.finalize();

    let formated_hash = format!("{:x}", hashed_data);

    let current_dir = utils::path_creator(None);
    
    let path_to_write: String = format!("{}/{}/objects/{}", current_dir, crate::MIGIT_DIR, formated_hash);

    match fs::write(path_to_write, &data) {
        Ok(_) => {
            println!("File created! Object {} created in {}", formated_hash, crate::MIGIT_DIR);
            0
        },
        Err(e) => {
            println!("Error writing file: {}", e);
            1
        }
    }
}

pub fn cat_file(path: &str) -> i8 {
    return match fs::read_to_string(path) {
        Ok(data_string) => {
            println!("{}", data_string);
            0
        },
        Err(e) => {
            println!("Error reading the file {}: {}", path, e);
            1
        }
    }
}