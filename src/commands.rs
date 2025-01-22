// Module: commands - Responsible for implementing the commands

use crate::data;

pub fn init_command() {
    data::init_directory();
}
