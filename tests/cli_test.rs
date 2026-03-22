use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_help() {
    let mut cmd = cargo_bin_cmd!("atb");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Agent ToolKit Builder"))
        .stdout(predicate::str::contains("Usage: atb"));
}

#[test]
fn test_build_default_config() {
    let mut cmd = cargo_bin_cmd!("atb");
    cmd.arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Config file not found: atb.yaml"));
}

#[test]
fn test_build_non_existent_config() {
    let mut cmd = cargo_bin_cmd!("atb");
    cmd.arg("build")
        .arg("--config")
        .arg("non_existent.yaml")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Config file not found: non_existent.yaml"));
}
