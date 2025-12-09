mod common;

use predicates::prelude::*;

// Vec<String> tests
#[test]
fn t_multiple_vec_with_multiple_values() {
    common::cli()
        .args(&["t_multiple_vec", "--v_vec", "first", "second", "third"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["first", "second", "third"]"#));
}

#[test]
fn t_multiple_vec_with_single_value() {
    common::cli()
        .args(&["t_multiple_vec", "--v_vec", "single"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["single"]"#));
}

#[test]
fn t_multiple_vec_with_empty_strings() {
    common::cli()
        .args(&["t_multiple_vec", "--v_vec", "", "test", ""])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"["", "test", ""]"#));
}

#[test]
fn t_multiple_vec_empty() {
    common::cli()
        .args(&["t_multiple_vec", "--v_vec"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

// Vec<i8> tests
#[test]
fn t_multiple_vec_int_with_multiple_values() {
    common::cli()
        .args(&["t_multiple_vec_int", "--v_vec", "1", "2", "3", "4", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[1, 2, 3, 4, 5]"));
}

#[test]
fn t_multiple_vec_int_single_value() {
    common::cli()
        .args(&["t_multiple_vec_int", "--v_vec", "42"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[42]"));
}

#[test]
fn t_multiple_vec_int_empty() {
    common::cli()
        .args(&["t_multiple_vec_int", "--v_vec"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

#[test]
fn t_multiple_vec_int_overflow() {
    common::cli()
        .args(&["t_multiple_vec_int", "--v_vec", "1", "128", "3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn t_multiple_vec_int_invalid_value() {
    common::cli()
        .args(&["t_multiple_vec_int", "--v_vec", "1", "not_a_number", "3"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

// RepeatMinMax<u16, 2, 5> tests
#[test]
fn t_multiple_min_max_with_minimum_values() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "10", "20"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[10, 20]"));
}

#[test]
fn t_multiple_min_max_with_three_values() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "10", "20", "30"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[10, 20, 30]"));
}

#[test]
fn t_multiple_min_max_with_maximum_values() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "1", "2", "3", "4", "5"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[1, 2, 3, 4, 5]"));
}

#[test]
fn t_multiple_min_max_too_few_values() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "10"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("requires at least 2 values").or(
            predicate::str::contains("2 values required")
        ));
}

#[test]
fn t_multiple_min_max_too_many_values() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "1", "2", "3", "4", "5", "6"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unexpected argument '6' found").or(
            predicate::str::contains("at most 5 values")
        ));
}

#[test]
fn t_multiple_min_max_with_zero() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "0", "0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[0, 0]"));
}

#[test]
fn t_multiple_min_max_with_max_u16() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "65535", "65535"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[65535, 65535]"));
}

#[test]
fn t_multiple_min_max_overflow() {
    common::cli()
        .args(&["t_multiple_min_max", "--v_vec", "65536", "100"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
