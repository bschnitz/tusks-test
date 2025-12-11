use assert_cmd::Command;

pub fn cli() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}
