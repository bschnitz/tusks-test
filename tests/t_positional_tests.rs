mod common;

use predicates::prelude::*;

// Single positional argument tests
#[test]
fn t_positional_with_simple_value() {
    common::cli()
        .args(&["t_positional", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn t_positional_with_spaces() {
    common::cli()
        .args(&["t_positional", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn t_positional_with_special_chars() {
    common::cli()
        .args(&["t_positional", "test-value_123!"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-value_123!"));
}

#[test]
fn t_positional_with_empty_string() {
    common::cli()
        .args(&["t_positional", ""])
        .assert()
        .success()
        .stdout(predicate::eq("").or(predicate::eq("\n")));
}

#[test]
fn t_positional_missing_argument() {
    common::cli()
        .arg("t_positional")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(
            predicate::str::contains("V_POSITIONAL")
        ));
}

#[test]
fn t_positional_help() {
    common::cli()
        .args(&["t_positional", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("<V_POSITIONAL>"))
        .stdout(predicate::str::contains("Arguments:"));
}

// Multiple positional arguments (Vec) tests
#[test]
fn t_positional_vec_with_multiple_values() {
    common::cli()
        .args(&["t_positional_vec", "first", "second", "third"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["first", "second", "third"]"#));
}

#[test]
fn t_positional_vec_with_single_value() {
    common::cli()
        .args(&["t_positional_vec", "single"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["single"]"#));
}

#[test]
fn t_positional_vec_with_spaces() {
    common::cli()
        .args(&["t_positional_vec", "hello world", "foo bar"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["hello world", "foo bar"]"#));
}

#[test]
fn t_positional_vec_with_empty_strings() {
    common::cli()
        .args(&["t_positional_vec", "", "middle", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["", "middle", ""]"#));
}

#[test]
fn t_positional_vec_with_many_values() {
    common::cli()
        .args(&["t_positional_vec", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]"#));
}

#[test]
fn t_positional_vec_help() {
    common::cli()
        .args(&["t_positional_vec", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[V_POSITIONAL]..."))
        .stdout(predicate::str::contains("Arguments:"));
}

// Test that positional args can't be mixed with flags
#[test]
fn t_positional_no_flag_syntax() {
    common::cli()
        .args(&["t_positional", "--v_positional", "value"])
        .assert()
        .failure();
}

#[test]
fn t_positional_vec_no_flag_syntax() {
    common::cli()
        .args(&["t_positional_vec", "--v_positional", "value"])
        .assert()
        .failure();
}
