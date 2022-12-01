use clap::{Parser};
use std::io;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf
}

fn write_to_file(path: std::path::PathBuf, data: String) -> Result<(), Box<dyn std::error::Error>>{
    let write = std::fs::write(path, data);

    match write {
        Ok(()) => {},
        Err(error) => {panic!("Could not make file! ERROR: {}", error);}, 
    };
    Ok(())
}

fn main() {
    let args = Cli::parse();

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Something went wrong.");


    write_to_file(args.path, user_input).expect("Could not write to file.");


}
