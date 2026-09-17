mod cli;
mod rootfs;

use std::{env, path::Path, process::Command};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    match cli::parse(&arguments) {
        Ok(cli::Command::Help) => println!("{}", cli::USAGE),
        Ok(cli::Command::Run {
            rootfs,
            program,
            arguments,
        }) => run_program(rootfs.as_deref(), &program, &arguments),
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

fn run_program(rootfs: Option<&Path>, program: &str, arguments: &[String]) -> ! {
    #[cfg(unix)]
    let mut command = match rootfs {
        Some(path) => {
            let mut command = Command::new("chroot");
            command.arg(path).arg(program);
            command
        }
        None => Command::new(program),
    };

    #[cfg(not(unix))]
    let mut command = match rootfs {
        Some(_) => {
            eprintln!("Running inside a rootfs is only supported on Unix.");
            std::process::exit(1);
        }
        None => Command::new(program),
    };

    let command_name = if rootfs.is_some() { "chroot" } else { program };
    let status = match command.args(arguments).status() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("Could not start '{command_name}': {error}");
            std::process::exit(1);
        }
    };

    std::process::exit(status.code().unwrap_or(1));
}
