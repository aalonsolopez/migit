use clap::Parser;

mod commands;
mod data;

#[derive(Parser)]
struct Cli {
    argument: String,
    path: String
}

fn main() {
    let args = Cli::parse();

    let error_flag: i8;

    match args.argument.as_str() {
        "init" => {
            error_flag = commands::init_command()
        },
        "hash-object" => {
            error_flag = commands::hash_object_command(&args.path)
        },
        _ => {
            println!("Unknown command");
            error_flag = 0;
        },
    }

    if error_flag == 1 {
        println!("Error during {} command.", args.argument)
    }
}