use std::path::PathBuf;

pub const USAGE: &str = "\
Usage:
  oxidize run [options] <program> [arguments...]
  oxidize init <rootfs-path>
  oxidize inspect <rootfs-path>

Run options:
  --rootfs <rootfs-path>  Use the directory as the program's filesystem root (Unix)
  --isolate               Run in new PID, mount, UTS and IPC namespaces (Linux)";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Run {
        rootfs: Option<PathBuf>,
        isolate: bool,
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
    let mut rootfs = None;
    let mut isolate = false;
    let mut program_index = 2;

    loop {
        match arguments.get(program_index).map(String::as_str) {
            Some("--rootfs") => {
                if rootfs.is_some() {
                    return Err("The --rootfs option can only be given once.");
                }
                let value = arguments
                    .get(program_index + 1)
                    .ok_or("A rootfs path is required.")?;
                if matches!(value.as_str(), "--rootfs" | "--isolate") {
                    return Err("A rootfs path is required.");
                }
                let path = PathBuf::from(value);
                rootfs = Some(path);
                program_index += 2;
            }
            Some("--isolate") => {
                isolate = true;
                program_index += 1;
            }
            _ => break,
        }
    }

    let program = arguments
        .get(program_index)
        .cloned()
        .ok_or("A program to run is required.")?;

    Ok(Command::Run {
        rootfs,
        isolate,
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
                isolate: false,
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
                isolate: false,
                program: "/bin/echo".to_owned(),
                arguments: vec!["hello".to_owned()],
            })
        );
    }

    #[test]
    fn parses_run_command_with_isolation() {
        assert_eq!(
            parse_values(&["oxidize", "run", "--isolate", "sh", "-c", "echo $$"]),
            Ok(Command::Run {
                rootfs: None,
                isolate: true,
                program: "sh".to_owned(),
                arguments: vec!["-c".to_owned(), "echo $$".to_owned()],
            })
        );
    }

    #[test]
    fn parses_run_options_in_any_order() {
        let expected = Ok(Command::Run {
            rootfs: Some(PathBuf::from("rootfs")),
            isolate: true,
            program: "/bin/sh".to_owned(),
            arguments: vec![],
        });

        assert_eq!(
            parse_values(&[
                "oxidize",
                "run",
                "--rootfs",
                "rootfs",
                "--isolate",
                "/bin/sh"
            ]),
            expected
        );
        assert_eq!(
            parse_values(&[
                "oxidize",
                "run",
                "--isolate",
                "--rootfs",
                "rootfs",
                "/bin/sh"
            ]),
            expected
        );
    }

    #[test]
    fn passes_options_after_the_program_to_the_program() {
        assert_eq!(
            parse_values(&["oxidize", "run", "echo", "--isolate"]),
            Ok(Command::Run {
                rootfs: None,
                isolate: false,
                program: "echo".to_owned(),
                arguments: vec!["--isolate".to_owned()],
            })
        );
    }

    #[test]
    fn rejects_repeated_rootfs_option() {
        assert_eq!(
            parse_values(&["oxidize", "run", "--rootfs", "a", "--rootfs", "b", "sh"]),
            Err("The --rootfs option can only be given once.")
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
        assert_eq!(
            parse_values(&["oxidize", "run", "--isolate"]),
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
