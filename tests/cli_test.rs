use assert_cmd::Command;
use colops::file_handling::{read_input_file, FileType};
use predicates::prelude::*;
use std::path::PathBuf;

#[test]
fn empty_command() {
    let mut cmd = Command::cargo_bin("colops").unwrap();
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: the following required arguments were not provided",
    ));
}

#[test]
fn valid_arguments() {
    let mut cmd = Command::cargo_bin("colops").unwrap();
    cmd.args(&["-f", "input.csv", "-c", "name", "-o", "distinct"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Input file: input.csv"))
        .stdout(predicate::str::contains("Column: name"))
        .stdout(predicate::str::contains("Operation: distinct"));
}

#[test]
fn test_process_csv_file() {
    let csv_file = PathBuf::from("tests/inputs/sample.csv");
    let (_, contents) = read_input_file(&csv_file).unwrap();
    assert!(contents.contains("col1,col2,col3"));
}
