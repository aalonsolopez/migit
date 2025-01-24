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

    println!("Initialized empty migit repository in {}{}", current_dir, crate::MIGIT_DIR);
    
    error_flag
}

pub fn hash_object(data: Vec<u8>, object_type: Option<&str>) -> i8 {
    // Adds blob and a 0x00 at the beginning of the doc
    let o_type = object_type.unwrap_or("blob");
    let mut content = o_type.as_bytes().to_vec();
    content.push(0);
    content.extend(data);

    // Hash stage
    let mut hasher = Sha1::new();
    hasher.update(&content);
    let hashed_data = hasher.finalize();

    let formated_hash = format!("{:x}", hashed_data);

    let current_dir = utils::path_creator(None);
    
    let path_to_write: String = format!("{}/{}/objects/{}", current_dir, crate::MIGIT_DIR, formated_hash);

    match fs::write(path_to_write, &content) {
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

fn get_object(hash: &str, expected_type: Option<&str>) -> Result<Vec<u8>, String> {
    let path = format!("{}/objects/{}", crate::MIGIT_DIR, hash);
    
    let content = fs::read(&path)
        .map_err(|e| format!("Error leyendo objeto: {}", e))?;
        
    // Separate type and content
    let null_pos = content.iter()
        .position(|&b| b == 0)
        .ok_or("Formato de objeto inválido")?;
        
    let tipo = String::from_utf8_lossy(&content[..null_pos]);
    
    // Verify if type is the expected type
    if let Some(expected) = expected_type {
        if tipo != expected {
            return Err(format!("Tipo incorrecto {} (esperado {})", tipo, expected));
        }
    }
    
    Ok(content[null_pos + 1..].to_vec())
}

pub fn cat_file(hash: &str) -> i8 {
    match get_object(hash, None) {
        Ok(data) => {
            println!("{}", String::from_utf8_lossy(&data));
            0
        },
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}