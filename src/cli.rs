use std::path::PathBuf;

pub const USAGE: &str = "\
Usage:
  oxidize run <program> [arguments...]
  oxidize run --rootfs <rootfs-path> <program> [arguments...]
  oxidize init <rootfs-path>
  oxidize inspect <rootfs-path>";

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
        Some("init") => parse_init(arguments),
        Some("inspect") => parse_inspect(arguments),
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

fn parse_init(arguments: &[String]) -> Result<Command, &'static str> {
    let path = parse_path_command(arguments, "init")?;
    Ok(Command::Init { path })
}

fn parse_inspect(arguments: &[String]) -> Result<Command, &'static str> {
    let path = parse_path_command(arguments, "inspect")?;
    Ok(Command::Inspect { path })
}

fn parse_path_command(arguments: &[String], command: &str) -> Result<PathBuf, &'static str> {
    let path = arguments
        .get(2)
        .map(PathBuf::from)
        .ok_or("A rootfs path is required.")?;

    if arguments.len() > 3 {
        return match command {
            "init" => Err("The init command accepts exactly one rootfs path."),
            "inspect" => Err("The inspect command accepts exactly one rootfs path."),
            _ => Err("The command accepts exactly one rootfs path."),
        };
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::{Command, parse};
    use std::path::PathBuf;

    fn arguments(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn parses_run_command() {
        let command = parse(&arguments(&["oxidize", "run", "echo", "hello"])).unwrap();

        match command {
            Command::Run {
                rootfs,
                program,
                arguments,
            } => {
                assert!(rootfs.is_none());
                assert_eq!(program, "echo");
                assert_eq!(arguments, ["hello"]);
            }
            Command::Init { .. } => panic!("expected run command"),
            Command::Inspect { .. } => panic!("expected run command"),
            Command::Help => panic!("expected run command"),
        }
    }

    #[test]
    fn parses_run_command_with_rootfs() {
        let command = parse(&arguments(&[
            "oxidize",
            "run",
            "--rootfs",
            "rootfs",
            "/bin/echo",
            "hello",
        ]))
        .unwrap();

        match command {
            Command::Run {
                rootfs,
                program,
                arguments,
            } => {
                assert_eq!(rootfs, Some(PathBuf::from("rootfs")));
                assert_eq!(program, "/bin/echo");
                assert_eq!(arguments, ["hello"]);
            }
            Command::Init { .. } => panic!("expected run command"),
            Command::Inspect { .. } => panic!("expected run command"),
            Command::Help => panic!("expected run command"),
        }
    }

    #[test]
    fn parses_init_command() {
        let command = parse(&arguments(&["oxidize", "init", "rootfs"])).unwrap();

        match command {
            Command::Init { path } => assert_eq!(path, PathBuf::from("rootfs")),
            Command::Run { .. } => panic!("expected init command"),
            Command::Inspect { .. } => panic!("expected init command"),
            Command::Help => panic!("expected init command"),
        }
    }

    #[test]
    fn parses_inspect_command() {
        let command = parse(&arguments(&["oxidize", "inspect", "rootfs"])).unwrap();

        match command {
            Command::Inspect { path } => assert_eq!(path, PathBuf::from("rootfs")),
            Command::Run { .. } => panic!("expected inspect command"),
            Command::Init { .. } => panic!("expected inspect command"),
            Command::Help => panic!("expected inspect command"),
        }
    }

    #[test]
    fn rejects_missing_init_path() {
        assert!(parse(&arguments(&["oxidize", "init"])).is_err());
    }
}
