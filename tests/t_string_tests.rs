mod common;

use predicates::prelude::*;

#[test]
fn t_string_with_simple_value() {
    common::cli()
        .args(&["t_string", "--v_string", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn t_string_with_spaces() {
    common::cli()
        .args(&["t_string", "--v_string", "hello world"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"));
}

#[test]
fn t_string_with_special_chars() {
    common::cli()
        .args(&["t_string", "--v_string", "test-with_special.chars!"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test-with_special.chars!"));
}

#[test]
fn t_string_missing_argument() {
    common::cli()
        .arg("t_string")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn t_string_help() {
    common::cli()
        .args(&["t_string", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--v_string"));
}

#[test]
fn t_int_with_positive_value() {
    common::cli()
        .args(&["t_int", "--v_int", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("42"));
}

#[test]
fn t_int_with_negative_value() {
    common::cli()
        .args(&["t_int", "--v_int=-15"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-15"));
}

#[test]
fn t_int_with_zero() {
    common::cli()
        .args(&["t_int", "--v_int", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
}

#[test]
fn t_int_with_max_i8() {
    common::cli()
        .args(&["t_int", "--v_int", "127"])
        .assert()
        .success()
        .stdout(predicate::str::contains("127"));
}

#[test]
fn t_int_with_min_i8() {
    common::cli()
        .args(&["t_int", "--v_int=-128"])
        .assert()
        .success()
        .stdout(predicate::str::contains("-128"));
}

#[test]
fn t_int_overflow() {
    common::cli()
        .args(&["t_int", "--v_int", "128"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_int_underflow() {
    common::cli()
        .args(&["t_int", "--v_int=-129"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_int_invalid_string() {
    common::cli()
        .args(&["t_int", "--v_int", "not_a_number"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_int_missing_argument() {
    common::cli()
        .arg("t_int")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}
