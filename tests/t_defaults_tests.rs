mod common;

use predicates::prelude::*;

// String defaults tests
#[test]
fn t_string_defaults_uses_default() {
    common::cli()
        .arg("t_string_defaults")
        .assert()
        .success()
        .stdout(predicate::str::contains("default"));
}

#[test]
fn t_string_defaults_with_custom_value() {
    common::cli()
        .args(&["t_string_defaults", "--v_default", "custom"])
        .assert()
        .success()
        .stdout(predicate::str::contains("custom"));
}

#[test]
fn t_string_defaults_with_empty_string() {
    common::cli()
        .args(&["t_string_defaults", "--v_default", ""])
        .assert()
        .success()
        .stdout(predicate::eq("").or(predicate::eq("\n")));
}

#[test]
fn t_string_defaults_with_special_chars() {
    common::cli()
        .args(&["t_string_defaults", "--v_default", "test!@#$%"])
        .assert()
        .success()
        .stdout(predicate::str::contains("test!@#$%"));
}

#[test]
fn t_string_defaults_help_shows_default() {
    common::cli()
        .args(&["t_string_defaults", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[default: default]"));
}

// Int defaults tests
#[test]
fn t_int_defaults_uses_default() {
    common::cli()
        .arg("t_int_defaults")
        .assert()
        .success()
        .stdout(predicate::str::contains("23"));
}

#[test]
fn t_int_defaults_with_custom_value() {
    common::cli()
        .args(&["t_int_defaults", "--v_default", "100"])
        .assert()
        .success()
        .stdout(predicate::str::contains("100"));
}

#[test]
fn t_int_defaults_with_zero() {
    common::cli()
        .args(&["t_int_defaults", "--v_default", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
}

#[test]
fn t_int_defaults_with_max_u8() {
    common::cli()
        .args(&["t_int_defaults", "--v_default", "255"])
        .assert()
        .success()
        .stdout(predicate::str::contains("255"));
}

#[test]
fn t_int_defaults_overflow() {
    common::cli()
        .args(&["t_int_defaults", "--v_default", "256"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_int_defaults_negative() {
    common::cli()
        .args(&["t_int_defaults", "--v_default=-1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_int_defaults_invalid_string() {
    common::cli()
        .args(&["t_int_defaults", "--v_default", "not_a_number"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_int_defaults_help_shows_default() {
    common::cli()
        .args(&["t_int_defaults", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[default: 23]"));
}
