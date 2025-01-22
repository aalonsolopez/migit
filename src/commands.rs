// Module: commands - Responsible for implementing the commands
use std::fs;

const MIGIT_DIR: &str = ".migit";

pub fn init_command() {
    fs::create_dir(MIGIT_DIR).expect("Failed to create .migit directory");
    println!("Initialized empty migit repository in .migit");
}
