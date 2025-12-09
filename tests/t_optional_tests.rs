mod common;

use predicates::prelude::*;

#[test]
fn t_optional_with_value() {
    common::cli()
        .args(&["t_optional", "--v_opt", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Some(42)"));
}

#[test]
fn t_optional_without_value() {
    common::cli()
        .arg("t_optional")
        .assert()
        .success()
        .stdout(predicate::str::contains("None"));
}

#[test]
fn t_optional_with_zero() {
    common::cli()
        .args(&["t_optional", "--v_opt", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Some(0)"));
}

#[test]
fn t_optional_with_max_u8() {
    common::cli()
        .args(&["t_optional", "--v_opt", "255"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Some(255)"));
}

#[test]
fn t_optional_overflow() {
    common::cli()
        .args(&["t_optional", "--v_opt", "256"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_optional_negative() {
    common::cli()
        .args(&["t_optional", "--v_opt=-1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_optional_vec_with_values() {
    common::cli()
        .args(&["t_optional_vec", "--v_vec", "foo", "bar", "baz"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"Some(["foo", "bar", "baz"])"#));
}

#[test]
fn t_optional_vec_with_single_value() {
    common::cli()
        .args(&["t_optional_vec", "--v_vec", "single"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"Some(["single"])"#));
}

#[test]
fn t_optional_vec_without_values() {
    common::cli()
        .arg("t_optional_vec")
        .assert()
        .success()
        .stdout(predicate::str::contains("None"));
}

#[test]
fn t_optional_vec_empty_flag() {
    // Test if --v_vec is provided without values
    common::cli()
        .args(&["t_optional_vec", "--v_vec"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Some([])"));
}
