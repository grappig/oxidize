use std::path::PathBuf;

pub const USAGE: &str = "\
Usage:
  oxidize run <program> [arguments...]
  oxidize init <rootfs-path>";

pub enum Command {
    Help,
    Run {
        program: String,
        arguments: Vec<String>,
    },
    Init {
        path: PathBuf,
    },
}

pub fn parse(arguments: &[String]) -> Result<Command, &'static str> {
    match arguments.get(1).map(String::as_str) {
        Some("run") => parse_run(arguments),
        Some("init") => parse_init(arguments),
        Some("--help") | Some("-h") | None => Ok(Command::Help),
        Some(_) => Err("Unknown command."),
    }
}

fn parse_run(arguments: &[String]) -> Result<Command, &'static str> {
    let program = arguments
        .get(2)
        .cloned()
        .ok_or("A program to run is required.")?;

    Ok(Command::Run {
        program,
        arguments: arguments[3..].to_vec(),
    })
}

fn parse_init(arguments: &[String]) -> Result<Command, &'static str> {
    let path = arguments
        .get(2)
        .map(PathBuf::from)
        .ok_or("A rootfs path is required.")?;

    if arguments.len() > 3 {
        return Err("The init command accepts exactly one rootfs path.");
    }

    Ok(Command::Init { path })
}

#[cfg(test)]
mod tests {
    use super::{parse, Command};
    use std::path::PathBuf;

    fn arguments(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn parses_run_command() {
        let command = parse(&arguments(&["oxidize", "run", "echo", "hello"])).unwrap();

        match command {
            Command::Run {
                program,
                arguments,
            } => {
                assert_eq!(program, "echo");
                assert_eq!(arguments, ["hello"]);
            }
            Command::Init { .. } => panic!("expected run command"),
            Command::Help => panic!("expected run command"),
        }
    }

    #[test]
    fn parses_init_command() {
        let command = parse(&arguments(&["oxidize", "init", "rootfs"])).unwrap();

        match command {
            Command::Init { path } => assert_eq!(path, PathBuf::from("rootfs")),
            Command::Run { .. } => panic!("expected init command"),
            Command::Help => panic!("expected init command"),
        }
    }

    #[test]
    fn rejects_missing_init_path() {
        assert!(parse(&arguments(&["oxidize", "init"])).is_err());
    }
}
