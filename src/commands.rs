// Module: commands - Responsible for implementing the commands

use std::fs;

use crate::{data, utils};

pub fn init_command() -> i8 {
    return data::init_directory(); 
}

pub fn hash_object_command(path: &str) -> i8 {
    let data_blob: Vec<u8>;

    match fs::read(path) {
        Ok(data) => {data_blob = data;},
        Err(e) =>  {
            println!("There has been an error reading the file: {}", e);
            return 1
        }
    }

    return data::hash_object(data_blob);
}

pub fn cat_file_command(file_name: &str) -> i8 {
    let data_blob: Vec<u8>;

    let formated_path = format!("/{}/objects/{}", crate::MIGIT_DIR, file_name);

    let path = utils::path_creator(Some(&formated_path));

    return data::cat_file(&path);
}