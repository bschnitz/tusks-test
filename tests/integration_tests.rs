// =============================================================================
// FILE: Cargo.toml (add to [dev-dependencies])
// =============================================================================
// [dev-dependencies]
// assert_cmd = "2.1.1"
// predicates = "3.1.3"

// =============================================================================
// FILE: tests/integration_tests.rs
// =============================================================================
use assert_cmd::Command;
use predicates::prelude::*;

/// Helper function to create a command with common setup
fn cmd() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

// =============================================================================
// ROOT LEVEL TESTS
// =============================================================================

#[test]
fn test_root_task1_with_parameters() {
    cmd()
        .args(&["--root-param", "test_root", "--verbose", "task1", "--arg1", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== root::task1 ==="))
        .stdout(predicate::str::contains("root_param: test_root"))
        .stdout(predicate::str::contains("verbose: true"))
        .stdout(predicate::str::contains("arg1: hello"))
        .stdout(predicate::str::contains("[VERBOSE] Executing task1 with root_param='test_root'"));
}

#[test]
fn test_root_task1_without_verbose() {
    cmd()
        .args(&["--root-param", "test_root", "task1", "--arg1", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("verbose: false"))
        .stdout(predicate::str::contains("[VERBOSE]").not());
}

#[test]
fn test_root_task2_return_value() {
    cmd()
        .args(&["--root-param", "test_root", "task2", "--value", "21"])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== root::task2 ==="))
        .stdout(predicate::str::contains("value: 21"))
        .stdout(predicate::str::contains("returning: 42"));
}

#[test]
fn test_root_task2_short_flag() {
    cmd()
        .args(&["--root-param", "test_root", "task2", "-v", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("value: 10"))
        .stdout(predicate::str::contains("returning: 20"));
}

#[test]
fn test_root_task3_no_args() {
    cmd()
        .args(&["--root-param", "test_root", "task3"])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== root::task3 ==="))
        .stdout(predicate::str::contains("No arguments, just executing"));
}

#[test]
fn test_root_task4_vec_params() {
    cmd()
        .args(&["--root-param", "test_root", "task4", "--items", "a", "--items", "b", "--items", "c"])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== root::task4 ==="))
        .stdout(predicate::str::contains("root_param: test_root"))
        .stdout(predicate::str::contains(r#"items: ["a", "b", "c"]"#))
        .stdout(predicate::str::contains("item count: 3"));
}

#[test]
fn test_root_task4_empty_vec() {
    cmd()
        .args(&["--root-param", "test_root", "task4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("items: []"))
        .stdout(predicate::str::contains("item count: 0"));
}

// =============================================================================
// LEVEL 1 TESTS
// =============================================================================

#[test]
fn test_level1_subtask1_with_super_access() {
    cmd()
        .args(&[
            "--root-param", "root_value",
            "--verbose",
            "level1",
            "--level1-field", "test_field",
            "subtask1",
            "--arg", "optional_value"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== level1::subtask1 ==="))
        .stdout(predicate::str::contains("level1_field: Some(\"test_field\")"))
        .stdout(predicate::str::contains("arg: Some(\"optional_value\")"))
        .stdout(predicate::str::contains("root_param (via super_): root_value"))
        .stdout(predicate::str::contains("verbose (via super_): true"))
        .stdout(predicate::str::contains("[VERBOSE] Level1 subtask1 accessing root_param='root_value'"));
}

#[test]
fn test_level1_subtask1_default_number() {
    cmd()
        .args(&[
            "--root-param", "root_value",
            "level1",
            "subtask1"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("level1_number: 42")); // default value
}

#[test]
fn test_level1_subtask1_custom_number() {
    cmd()
        .args(&[
            "--root-param", "root_value",
            "level1",
            "--level1-number", "100",
            "subtask1"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("level1_number: 100"));
}

#[test]
fn test_level1_subtask2_super_chain() {
    cmd()
        .args(&[
            "--root-param", "chained_root",
            "--verbose",
            "level1",
            "--level1-field", "l1_value",
            "subtask2"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== level1::subtask2 ==="))
        .stdout(predicate::str::contains("level1_field: Some(\"l1_value\")"))
        .stdout(predicate::str::contains("root_param: chained_root"))
        .stdout(predicate::str::contains("verbose: true"));
}

// =============================================================================
// LEVEL 2 TESTS (Deep Nesting)
// =============================================================================

#[test]
fn test_level2_deep_task_super_chain() {
    cmd()
        .args(&[
            "--root-param", "deep_root",
            "--verbose",
            "level1",
            "--level1-field", "l1",
            "--level1-number", "99",
            "level2",
            "--level2-id", "12345",
            "deep_task",
            "--enabled"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== level2::deep_task ==="))
        .stdout(predicate::str::contains("level2_id: 12345"))
        .stdout(predicate::str::contains("enabled: true"))
        .stdout(predicate::str::contains("level1_field (via super_): Some(\"l1\")"))
        .stdout(predicate::str::contains("level1_number (via super_): 99"))
        .stdout(predicate::str::contains("root_param (via super_.super_): deep_root"))
        .stdout(predicate::str::contains("verbose (via super_.super_): true"))
        .stdout(predicate::str::contains("[VERBOSE] Deep in level2, can still access root!"));
}

#[test]
fn test_level2_deep_task_without_verbose() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1",
            "level2",
            "--level2-id", "999",
            "deep_task"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("enabled: false"))
        .stdout(predicate::str::contains("[VERBOSE]").not());
}

// =============================================================================
// LEVEL 3 TESTS (Maximum Depth)
// =============================================================================

#[test]
fn test_level3_very_deep() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1",
            "level2",
            "--level2-id", "123",
            "level3",
            "very_deep",
            "--depth", "42"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== level3::very_deep ==="))
        .stdout(predicate::str::contains("depth: 42"))
        .stdout(predicate::str::contains("Nested 42 levels deep!"));
}

// =============================================================================
// LEVEL 1B TESTS (No Parameters Struct)
// =============================================================================

#[test]
fn test_level1b_task_no_params() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1b",
            "task_no_params",
            "--x", "255"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== level1b::task_no_params ==="))
        .stdout(predicate::str::contains("x: 255"))
        .stdout(predicate::str::contains("No Parameters struct in this module"));
}

#[test]
fn test_level1b_task_multi_args_all_options() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1b",
            "task_multi_args",
            "--name", "Alice",
            "--age", "30",
            "-a", // short flag for active
            "--tags", "rust",
            "--tags", "cli",
            "--tags", "testing"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== level1b::task_multi_args ==="))
        .stdout(predicate::str::contains("name: Alice"))
        .stdout(predicate::str::contains("age: Some(30)"))
        .stdout(predicate::str::contains("active: true"))
        .stdout(predicate::str::contains(r#"tags: ["rust", "cli", "testing"]"#));
}

#[test]
fn test_level1b_task_multi_args_optional_missing() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1b",
            "task_multi_args",
            "-n", "Bob", // short flag for name
            "-a"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("name: Bob"))
        .stdout(predicate::str::contains("age: None"))
        .stdout(predicate::str::contains("active: true"))
        .stdout(predicate::str::contains("tags: []"));
}

// =============================================================================
// EXTERNAL ROOT MODULE TESTS
// =============================================================================

#[test]
fn test_extroot_ext_task_with_parent_access() {
    cmd()
        .args(&[
            "--root-param", "main_root",
            "--verbose",
            "extroot",
            "--extroot-name", "external_module",
            "ext_task",
            "--count", "100"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== external_root::ext_task ==="))
        .stdout(predicate::str::contains("extroot_name: external_module"))
        .stdout(predicate::str::contains("count: 100"))
        .stdout(predicate::str::contains("root_param (via parent_): main_root"))
        .stdout(predicate::str::contains("verbose (via parent_): true"))
        .stdout(predicate::str::contains("[VERBOSE] External module accessing root parameters!"));
}

#[test]
fn test_extroot_sub_ext_sub_task() {
    cmd()
        .args(&[
            "--root-param", "root",
            "extroot",
            "--extroot-name", "ext",
            "sub",
            "ext_sub_task",
            "--msg", "hello_from_sub"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== external_root::sub::ext_sub_task ==="))
        .stdout(predicate::str::contains("msg: hello_from_sub"));
}

// =============================================================================
// EXTERNAL1 MODULE TESTS (Nested under level1)
// =============================================================================

#[test]
fn test_ext1_task_deep_chain() {
    cmd()
        .args(&[
            "--root-param", "root_val",
            "--verbose",
            "level1",
            "--level1-field", "l1_val",
            "--level1-number", "77",
            "ext1",
            "--ext1-param", "123",
            "ext1_task"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== external1::ext1_task ==="))
        .stdout(predicate::str::contains("ext1_param: Some(123)"))
        .stdout(predicate::str::contains("level1_field (via parent_): Some(\"l1_val\")"))
        .stdout(predicate::str::contains("level1_number (via parent_): 77"))
        .stdout(predicate::str::contains("root_param (via parent_.super_): root_val"))
        .stdout(predicate::str::contains("verbose (via parent_.super_): true"))
        .stdout(predicate::str::contains("[VERBOSE] external1 can access root through level1!"));
}

#[test]
fn test_ext1_vec_task() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1",
            "ext1",
            "ext1_vec_task",
            "--values", "10",
            "--values", "20",
            "--values", "30"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== external1::ext1_vec_task ==="))
        .stdout(predicate::str::contains("values: [10, 20, 30]"))
        .stdout(predicate::str::contains("sum: 60"))
        .stdout(predicate::str::contains("Chain to root: ext1 -> level1 -> root"));
}

// =============================================================================
// EXTERNAL2 MODULE TESTS (Deeply nested external)
// =============================================================================

#[test]
fn test_ext2_task_maximum_chain() {
    cmd()
        .args(&[
            "--root-param", "deepest_root",
            "--verbose",
            "level1",
            "--level1-field", "level1_data",
            "ext1",
            "--ext1-param", "999",
            "ext2",
            "--ext2-param", "ext2_data",
            "ext2_task",
            "-x", "test_value"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== external2::ext2_task ==="))
        .stdout(predicate::str::contains("ext2_param: ext2_data"))
        .stdout(predicate::str::contains("x: test_value"))
        .stdout(predicate::str::contains("ext1_param (via parent_): Some(999)"))
        .stdout(predicate::str::contains("level1_field (via parent_.super_): Some(\"level1_data\")"))
        .stdout(predicate::str::contains("root_param (via parent_.super_.super_): deepest_root"))
        .stdout(predicate::str::contains("Parameter chain: ext2 -> ext1 -> level1 -> root"))
        .stdout(predicate::str::contains("Depth: 4 levels!"))
        .stdout(predicate::str::contains("[VERBOSE] ext2 accessing root 3 levels up!"));
}

#[test]
fn test_ext2_complex_full_chain() {
    cmd()
        .args(&[
            "--root-param", "root",
            "--verbose",
            "level1",
            "ext1",
            "ext2",
            "--ext2-param", "complex",
            "ext2_complex",
            "--flag",
            "--numbers", "100",
            "--numbers", "200",
            "--numbers", "300",
            "-o", "optional_data"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("=== external2::ext2_complex ==="))
        .stdout(predicate::str::contains("ext2_param: complex"))
        .stdout(predicate::str::contains("flag: true"))
        .stdout(predicate::str::contains("numbers: [100, 200, 300]"))
        .stdout(predicate::str::contains("optional: Some(\"optional_data\")"))
        .stdout(predicate::str::contains("Full parameter chain access:"))
        .stdout(predicate::str::contains("ext2_param: complex"))
        .stdout(predicate::str::contains("root_param: root"));
}

#[test]
fn test_ext2_complex_minimal_args() {
    cmd()
        .args(&[
            "--root-param", "root",
            "level1",
            "ext1",
            "ext2",
            "--ext2-param", "minimal",
            "ext2_complex"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("flag: false"))
        .stdout(predicate::str::contains("numbers: []"))
        .stdout(predicate::str::contains("optional: None"));
}

// =============================================================================
// ERROR TESTS
// =============================================================================

#[test]
fn test_missing_required_root_param() {
    cmd()
        .args(&["task1", "--arg1", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_invalid_command() {
    cmd()
        .args(&["--root-param", "root", "nonexistent-command"])
        .assert()
        .failure();
}

#[test]
fn test_missing_required_arg_in_task() {
    cmd()
        .args(&["--root-param", "root", "task1"]) // missing --arg1
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_invalid_type_for_numeric_arg() {
    cmd()
        .args(&["--root-param", "root", "task2", "--value", "not_a_number"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid"));
}

// =============================================================================
// HELP TESTS
// =============================================================================

#[test]
fn test_root_help() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--root-param"))
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains("task1"))
        .stdout(predicate::str::contains("task2"))
        .stdout(predicate::str::contains("level1"))
        .stdout(predicate::str::contains("extroot"));
}

#[test]
fn test_level1_help() {
    cmd()
        .args(&["--root-param", "root", "level1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--level1-field"))
        .stdout(predicate::str::contains("--level1-number"))
        .stdout(predicate::str::contains("subtask1"))
        .stdout(predicate::str::contains("subtask2"))
        .stdout(predicate::str::contains("level2"))
        .stdout(predicate::str::contains("ext1"));
}

#[test]
fn test_task_help() {
    cmd()
        .args(&["--root-param", "root", "task1", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--arg1"));
}

// =============================================================================
// COMBINATION TESTS
// =============================================================================

#[test]
fn test_verbose_propagates_through_all_levels() {
    // Test that verbose flag set at root is accessible in deeply nested functions
    cmd()
        .args(&[
            "--root-param", "propagate_test",
            "--verbose",
            "level1",
            "level2",
            "--level2-id", "1",
            "deep_task",
            "--enabled"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("[VERBOSE] Deep in level2, can still access root!"));
}

#[test]
fn test_parameter_chain_integrity() {
    // Verify that parameter chain remains intact through multiple levels
    cmd()
        .args(&[
            "--root-param", "chain_root",
            "level1",
            "--level1-field", "chain_l1",
            "ext1",
            "--ext1-param", "456",
            "ext2",
            "--ext2-param", "chain_ext2",
            "ext2_task",
            "-x", "chain_test"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("root_param (via parent_.super_.super_): chain_root"))
        .stdout(predicate::str::contains("level1_field (via parent_.super_): Some(\"chain_l1\")"))
        .stdout(predicate::str::contains("ext1_param (via parent_): Some(456)"))
        .stdout(predicate::str::contains("ext2_param: chain_ext2"));
}
