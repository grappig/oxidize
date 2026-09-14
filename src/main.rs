use std::{env, process::Command};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.get(1).map(String::as_str) != Some("run") || arguments.len() < 3 {
        eprintln!("Usage: oxidize run <program> [arguments...]");
        std::process::exit(2);
    }

    let status = match Command::new(&arguments[2])
        .args(&arguments[3..])
        .status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Could not start '{}': {error}", arguments[2]);
            std::process::exit(1);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}