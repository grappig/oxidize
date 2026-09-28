use std::path::PathBuf;

pub const USAGE: &str = "\
Usage:
  oxidize run <program> [arguments...]
  oxidize run --rootfs <rootfs-path> <program> [arguments...]
  oxidize init <rootfs-path>
  oxidize inspect <rootfs-path>";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Run {
        rootfs: Option<PathBuf>,
        program: String,
        arguments: Vec<String>,
    },
    Init {
        path: PathBuf,
    },
    Inspect {
        path: PathBuf,
    },
}

pub fn parse(arguments: &[String]) -> Result<Command, &'static str> {
    match arguments.get(1).map(String::as_str) {
        Some("run") => parse_run(arguments),
        Some("init") => {
            let path = parse_rootfs_path(
                arguments,
                "The init command accepts exactly one rootfs path.",
            )?;
            Ok(Command::Init { path })
        }
        Some("inspect") => {
            let path = parse_rootfs_path(
                arguments,
                "The inspect command accepts exactly one rootfs path.",
            )?;
            Ok(Command::Inspect { path })
        }
        Some("--help") | Some("-h") | None => Ok(Command::Help),
        Some(_) => Err("Unknown command."),
    }
}

fn parse_run(arguments: &[String]) -> Result<Command, &'static str> {
    let (rootfs, program_index) = match arguments.get(2).map(String::as_str) {
        Some("--rootfs") => {
            let path = arguments
                .get(3)
                .map(PathBuf::from)
                .ok_or("A rootfs path is required.")?;
            (Some(path), 4)
        }
        _ => (None, 2),
    };

    let program = arguments
        .get(program_index)
        .cloned()
        .ok_or("A program to run is required.")?;

    Ok(Command::Run {
        rootfs,
        program,
        arguments: arguments[program_index + 1..].to_vec(),
    })
}

fn parse_rootfs_path(
    arguments: &[String],
    too_many_arguments: &'static str,
) -> Result<PathBuf, &'static str> {
    let path = arguments
        .get(2)
        .map(PathBuf::from)
        .ok_or("A rootfs path is required.")?;

    if arguments.len() > 3 {
        return Err(too_many_arguments);
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::{Command, parse};
    use std::path::PathBuf;

    fn parse_values(values: &[&str]) -> Result<Command, &'static str> {
        let arguments: Vec<String> = values.iter().map(|value| (*value).to_owned()).collect();
        parse(&arguments)
    }

    #[test]
    fn parses_help() {
        assert_eq!(parse_values(&["oxidize"]), Ok(Command::Help));
        assert_eq!(parse_values(&["oxidize", "--help"]), Ok(Command::Help));
        assert_eq!(parse_values(&["oxidize", "-h"]), Ok(Command::Help));
    }

    #[test]
    fn parses_run_command() {
        assert_eq!(
            parse_values(&["oxidize", "run", "echo", "hello"]),
            Ok(Command::Run {
                rootfs: None,
                program: "echo".to_owned(),
                arguments: vec!["hello".to_owned()],
            })
        );
    }

    #[test]
    fn parses_run_command_with_rootfs() {
        assert_eq!(
            parse_values(&["oxidize", "run", "--rootfs", "rootfs", "/bin/echo", "hello",]),
            Ok(Command::Run {
                rootfs: Some(PathBuf::from("rootfs")),
                program: "/bin/echo".to_owned(),
                arguments: vec!["hello".to_owned()],
            })
        );
    }

    #[test]
    fn parses_init_command() {
        assert_eq!(
            parse_values(&["oxidize", "init", "rootfs"]),
            Ok(Command::Init {
                path: PathBuf::from("rootfs"),
            })
        );
    }

    #[test]
    fn parses_inspect_command() {
        assert_eq!(
            parse_values(&["oxidize", "inspect", "rootfs"]),
            Ok(Command::Inspect {
                path: PathBuf::from("rootfs"),
            })
        );
    }

    #[test]
    fn rejects_unknown_command() {
        assert_eq!(
            parse_values(&["oxidize", "launch"]),
            Err("Unknown command.")
        );
    }

    #[test]
    fn rejects_run_without_program() {
        assert_eq!(
            parse_values(&["oxidize", "run"]),
            Err("A program to run is required.")
        );
        assert_eq!(
            parse_values(&["oxidize", "run", "--rootfs", "rootfs"]),
            Err("A program to run is required.")
        );
    }

    #[test]
    fn rejects_run_with_missing_rootfs_path() {
        assert_eq!(
            parse_values(&["oxidize", "run", "--rootfs"]),
            Err("A rootfs path is required.")
        );
    }

    #[test]
    fn rejects_missing_init_path() {
        assert_eq!(
            parse_values(&["oxidize", "init"]),
            Err("A rootfs path is required.")
        );
    }

    #[test]
    fn rejects_extra_rootfs_paths() {
        assert_eq!(
            parse_values(&["oxidize", "init", "a", "b"]),
            Err("The init command accepts exactly one rootfs path.")
        );
        assert_eq!(
            parse_values(&["oxidize", "inspect", "a", "b"]),
            Err("The inspect command accepts exactly one rootfs path.")
        );
    }
}
