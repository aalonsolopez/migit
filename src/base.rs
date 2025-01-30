use std::{fs, path};

// Write the tree of the files in the given path
pub fn write_tree(path: &str) -> i8 {
    let path = path::Path::new(path);

    match fs::read_dir(path) {
        Ok(files) => {
            for file in files {
                let file = file.unwrap();
                let file_name = file.file_name().into_string().unwrap();
                let file_type = file.file_type().unwrap();

                if file_type.is_dir() {
                    write_tree(path.join(&file_name).to_str().unwrap());
                } else {
                    println!("File: {}", file_name);
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