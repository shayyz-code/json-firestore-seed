use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_not_an_array_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("tests/fixtures/not_an_array.json")
        .arg("--collection")
        .arg("test")
        .arg("--project")
        .arg("test-project");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("JSON must be an array of objects"));
}

#[test]
fn test_invalid_timestamp_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("tests/fixtures/invalid_timestamp.json")
        .arg("--collection")
        .arg("test")
        .arg("--project")
        .arg("test-project")
        .arg("--dry-run");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Timestamp parsing failed"));
}

#[test]
fn test_file_not_found_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("non_existent.json")
        .arg("--collection")
        .arg("test")
        .arg("--project")
        .arg("test-project");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_empty_collection_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("tests/fixtures/valid_users.json")
        .arg("--collection")
        .arg("")
        .arg("--project")
        .arg("test-project");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Collection name cannot be empty"));
}

#[test]
fn test_missing_id_field_error() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("tests/fixtures/valid_users.json")
        .arg("--collection")
        .arg("test")
        .arg("--project")
        .arg("test-project")
        .arg("--id-field")
        .arg("non_existent_field")
        .arg("--dry-run");

    cmd.assert().failure().stderr(predicate::str::contains(
        "ID field 'non_existent_field' not found or invalid",
    ));
}

#[test]
fn test_dry_run_output() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("tests/fixtures/valid_users.json")
        .arg("--collection")
        .arg("test")
        .arg("--project")
        .arg("test-project")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Dry Run: Document ID"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}

#[test]
fn test_batch_size_dry_run() {
    let mut cmd = Command::cargo_bin("json-firestore-seed").unwrap();
    cmd.arg("--json")
        .arg("tests/fixtures/valid_users.json")
        .arg("--collection")
        .arg("test")
        .arg("--project")
        .arg("test-project")
        .arg("--batch-size")
        .arg("2")
        .arg("--dry-run");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Dry Run: Document ID"))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));
}
