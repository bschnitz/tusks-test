use assert_cmd::Command;

/// Helper function to create a new Command for the binary
pub fn cli() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

/// Helper to check if output contains expected text
pub fn assert_output_contains(cmd: &mut Command, expected: &str) {
    cmd.assert()
        .success()
        .stdout(predicates::str::contains(expected));
}
