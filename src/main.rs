use clap::Parser;

mod commands;
mod data;

#[derive(Parser)]
struct Cli {
    argument: String
}

fn main() {
    let args = Cli::parse();

    match args.argument.as_str() {
        "init" => commands::init_command(),
        _ => println!("Unknown command"),
    }
}