use std::process::{Command, Output};

fn oxidize(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_oxidize"))
        .args(arguments)
        .output()
        .expect("failed to start oxidize")
}

#[cfg(target_os = "linux")]
#[test]
#[ignore = "creating namespaces requires root"]
fn isolated_program_runs_as_pid_one() {
    let output = oxidize(&["run", "--isolate", "sh", "-c", "echo $$"]);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "1");
}

#[cfg(not(target_os = "linux"))]
#[test]
fn isolation_is_rejected_outside_linux() {
    let output = oxidize(&["run", "--isolate", "program"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("Namespace isolation is only supported on Linux.")
    );
}
