mod cli;
mod rootfs;

use std::{env, process::Command};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    match cli::parse(&arguments) {
        Ok(cli::Command::Help) => println!("{}", cli::USAGE),
        Ok(cli::Command::Run {
               program,
               arguments,
           }) => run_program(&program, &arguments),
        Ok(cli::Command::Init { path }) => match rootfs::initialize(&path) {
            Ok(()) => println!("Created rootfs layout at '{}'.", path.display()),
            Err(error) => {
                eprintln!("Could not create rootfs '{}': {error}", path.display());
                std::process::exit(1);
            }
        },
        Ok(cli::Command::Inspect { path }) => match rootfs::inspect(&path) {
            Ok(report) if report.is_valid() => {
                println!("Rootfs '{}' is valid.", path.display());
            }
            Ok(report) => {
                println!("Rootfs '{}' is incomplete.", path.display());
                println!("Missing directories:");
                for directory in report.missing_directories() {
                    println!("  - {directory}");
                }
                std::process::exit(1);
            }
            Err(error) => {
                eprintln!("Could not inspect rootfs '{}': {error}", path.display());
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("{error}\n\n{}", cli::USAGE);
            std::process::exit(2);
        }
    }
}

fn run_program(program: &str, arguments: &[String]) -> ! {
    let status = match Command::new(program).args(arguments).status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Could not start '{program}': {error}");
            std::process::exit(1);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}
