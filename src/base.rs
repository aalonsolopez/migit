use std::{fs, path};

use crate::{data, utils::is_ignored};

// Write the tree of the files in the given path
pub fn write_tree(path: &str) -> i8 {
    let path = path::Path::new(path);

    match fs::read_dir(path) {
        Ok(files) => {
            for file in files {
                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap();
                let file_type = file.file_type().unwrap();

                if is_ignored(path.to_path_buf()) {
                    continue;
                }

                if file_type.is_dir() {
                    write_tree(path.join(&file_name).to_str().unwrap());
                } else {
                    let file_content = fs::read(file.path()).unwrap();
                    println!("File named {} content hashed: {}", file_name, data::hash_object_without_saving(file_content, Some("blob")));
                }   
            }
        },
        Err(e) => {
            println!("There has been an error reading the directory: {}", e);
            return 1
        }
    }
    return 0
}