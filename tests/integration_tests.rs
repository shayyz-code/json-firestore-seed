use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_not_an_array_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json").arg("tests/fixtures/not_an_array.json")
       .arg("--collection").arg("test")
       .arg("--project").arg("test-project");

    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("JSON must be an array of objects"));
}

#[test]
fn test_invalid_timestamp_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json").arg("tests/fixtures/invalid_timestamp.json")
       .arg("--collection").arg("test")
       .arg("--project").arg("test-project");

    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("Timestamp parsing failed"));
}

#[test]
fn test_file_not_found_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json").arg("non_existent.json")
       .arg("--collection").arg("test")
       .arg("--project").arg("test-project");

    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_empty_collection_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json").arg("tests/fixtures/valid_users.json")
       .arg("--collection").arg("")
       .arg("--project").arg("test-project");

    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("Collection name cannot be empty"));
}
