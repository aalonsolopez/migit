use clap::{Parser, Subcommand};

mod commands;
mod data;
mod utils;
mod base;

pub const MIGIT_DIR: &'static str = ".migit";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    
    HashObject {
        #[arg(required = true)]
        path: String,
    },

    CatFile {
        #[arg(required=true)]
        file_name: String,
    },
    WriteTree {
        #[arg(required=true)]
        path: String,
    }
}

fn main() {
    let cli = Cli::parse();

    let error_flag = match cli.command {
        Commands::Init => commands::init_command(),
        Commands::HashObject { path } => {
            commands::hash_object_command(&path)
        },
        Commands::CatFile { file_name } => {
            commands::cat_file_command(&file_name)
        },
        Commands::WriteTree { path } => {
            commands::write_tree(&path)
        }
    };

    if error_flag == 1 {
        println!("Error durante la ejecución del comando")
    }
}