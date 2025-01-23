// Module utils - Contains some utility functions

pub fn path_creator(path: Option<&str>) -> String {
    let path = path.unwrap_or("");
    let current_dir = match std::env::current_dir() {
        Ok(dir) => {
            dir.display().to_string()
        },
        Err(e) => {
            panic!("Failed to get current directory: {}", e)
        }
    };

    format!("{}/{}", current_dir, path)
}