use clap::Parser;

mod commands;

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