mod common;

use predicates::prelude::*;

// Submodule within main tasks module
#[test]
fn t_submodule_basic() {
    common::cli()
        .args(&["submodule.t_submodule"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Submodule"));
}

#[test]
fn t_submodule_help() {
    common::cli()
        .args(&["submodule.t_submodule", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

// Imported submodule (submod1)
#[test]
fn t_submod1_basic() {
    common::cli()
        .args(&["submod1.t_submod1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Submodule 1"));
}

#[test]
fn t_submod1_help() {
    common::cli()
        .args(&["submod1.t_submod1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

// Nested imported submodule (submod1.submod2)
#[test]
fn t_submod2_basic() {
    common::cli()
        .args(&["submod1.submod2.t_submod2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Submodule 2"));
}

#[test]
fn t_submod2_help() {
    common::cli()
        .args(&["submod1.submod2.t_submod2", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

// Test that submodules are listed in main help
#[test]
fn main_help_lists_submodules() {
    common::cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("submodule.t_submodule"))
        .stdout(predicate::str::contains("submod1.t_submod1"))
        .stdout(predicate::str::contains("submod1.submod2.t_submod2"));
}

// Test invalid submodule paths
#[test]
fn invalid_submodule_path() {
    common::cli()
        .args(&["nonexistent.command"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized").or(
            predicate::str::contains("invalid")
        ));
}

#[test]
fn incomplete_submodule_path() {
    common::cli()
        .arg("submod1")
        .assert()
        .failure();
}
