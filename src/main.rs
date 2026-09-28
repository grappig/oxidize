mod cli;
#[cfg(target_os = "linux")]
mod namespaces;
mod rootfs;

use std::{env, path::Path, process::Command};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    match cli::parse(&arguments) {
        Ok(cli::Command::Help) => println!("{}", cli::USAGE),
        Ok(cli::Command::Run {
            rootfs,
            isolate,
            program,
            arguments,
        }) => run_program(rootfs.as_deref(), isolate, &program, &arguments),
        Ok(cli::Command::Init { path }) => initialize_rootfs(&path),
        Ok(cli::Command::Inspect { path }) => inspect_rootfs(&path),
        Err(error) => {
            eprintln!("{error}\n\n{}", cli::USAGE);
            std::process::exit(2);
        }
    }
}

fn fail(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}

fn initialize_rootfs(path: &Path) {
    if let Err(error) = rootfs::initialize(path) {
        fail(&format!(
            "Could not create rootfs '{}': {error}",
            path.display()
        ));
    }

    println!("Created rootfs layout at '{}'.", path.display());
}

fn inspect_rootfs(path: &Path) {
    let report = rootfs::inspect(path).unwrap_or_else(|error| {
        fail(&format!(
            "Could not inspect rootfs '{}': {error}",
            path.display()
        ))
    });

    if report.is_valid() {
        println!("Rootfs '{}' is valid.", path.display());
        return;
    }

    println!("Rootfs '{}' is incomplete.", path.display());
    println!("Missing directories:");
    for directory in report.missing_directories() {
        println!("  - {directory}");
    }
    std::process::exit(1);
}

fn run_program(rootfs: Option<&Path>, isolate: bool, program: &str, arguments: &[String]) -> ! {
    if isolate {
        enter_new_namespaces();
    }

    let (mut command, command_name) = match rootfs {
        Some(path) => (rootfs_command(path, program), "chroot"),
        None => (Command::new(program), program),
    };

    let status = command.args(arguments).status().unwrap_or_else(|error| {
        fail(&format!("Could not start '{command_name}': {error}"));
    });

    std::process::exit(status.code().unwrap_or(1));
}

#[cfg(unix)]
fn rootfs_command(rootfs: &Path, program: &str) -> Command {
    let mut command = Command::new("chroot");
    command.arg(rootfs).arg(program);
    command
}

#[cfg(not(unix))]
fn rootfs_command(_rootfs: &Path, _program: &str) -> Command {
    fail("Running inside a rootfs is only supported on Unix.");
}

#[cfg(target_os = "linux")]
fn enter_new_namespaces() {
    if let Err(error) = namespaces::unshare() {
        fail(&format!("Could not create namespaces: {error}"));
    }
}

#[cfg(not(target_os = "linux"))]
fn enter_new_namespaces() {
    fail("Namespace isolation is only supported on Linux.");
}
