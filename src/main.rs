use clap::{Parser, Subcommand};

mod commands;
mod data;
mod utils;

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
}

fn main() {
    let cli = Cli::parse();

    let error_flag = match cli.command {
        Commands::Init => commands::init_command(),
        Commands::HashObject { path } => {
            commands::hash_object_command(&path)
        }
    };

    if error_flag == 1 {
        println!("Error durante la ejecución del comando")
    }
}