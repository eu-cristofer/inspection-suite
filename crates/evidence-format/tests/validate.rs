#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::{Manifest, ValidationIssue, from_json, validate};
use serde_json::{Value, json};

const SNAPSHOT: &str = include_str!("fixtures/snapshot_v1_1.json");
const PUMP: &str = include_str!("fixtures/pump_test_v1_0.json");

fn mutated(text: &str, edits: &[(&str, Value)]) -> Manifest {
    let mut v: Value = serde_json::from_str(text).expect("value");
    for (pointer, new) in edits {
        *v.pointer_mut(pointer).expect("pointer exists") = new.clone();
    }
    from_json(&v.to_string()).expect("parse")
}

fn issues_of(m: &Manifest) -> Vec<ValidationIssue> {
    validate(m).expect_err("expected issues")
}

fn has_pointer(issues: &[ValidationIssue], pointer: &str) -> bool {
    issues.iter().any(|i| i.pointer == pointer)
}

#[test]
fn fixtures_validate() {
    assert!(validate(&from_json(SNAPSHOT).expect("parse")).is_ok());
    assert!(validate(&from_json(PUMP).expect("parse")).is_ok());
}

fn bad_image_path(path: &str) {
    let m = mutated(SNAPSHOT, &[("/items/0/image/path", json!(path))]);
    let issues = issues_of(&m);
    assert!(has_pointer(&issues, "/items/0/image/path"), "{issues:?}");
}

#[test]
fn rejects_parent_dir() {
    bad_image_path("../x.png");
}

#[test]
fn rejects_absolute() {
    bad_image_path("/abs.png");
}

#[test]
fn rejects_backslash() {
    bad_image_path("evidence\\x.png");
}

#[test]
fn rejects_double_slash() {
    bad_image_path("evidence//x.png");
}

#[test]
fn rejects_dot_prefix() {
    bad_image_path("./evidence/x.png");
}

#[test]
fn rejects_bad_path_in_nested_pump_report_key() {
    let m = mutated(PUMP, &[("/reports/pdf_path", json!("../report.pdf"))]);
    let issues = issues_of(&m);
    assert!(has_pointer(&issues, "/reports/pdf_path"), "{issues:?}");
}

#[test]
fn rejects_duplicate_id() {
    let m = mutated(SNAPSHOT, &[("/items/1/id", json!("item_1"))]);
    let issues = issues_of(&m);
    assert!(has_pointer(&issues, "/items/1/id"), "{issues:?}");
}

#[test]
fn rejects_out_of_order() {
    let m = mutated(SNAPSHOT, &[("/items/0/id", json!("item_2")), ("/items/1/id", json!("item_1"))]);
    let issues = issues_of(&m);
    assert!(has_pointer(&issues, "/items/0/id"), "{issues:?}");
}

#[test]
fn rejects_unknown_kind() {
    let m = mutated(SNAPSHOT, &[("/items/0/kind", json!("chart"))]);
    let issues = issues_of(&m);
    assert!(has_pointer(&issues, "/items/0/kind"), "{issues:?}");
}

#[test]
fn rejects_unsupported_version() {
    let m = mutated(PUMP, &[("/manifest_version", json!("9.9"))]);
    let issues = issues_of(&m);
    assert!(has_pointer(&issues, "/manifest_version"), "{issues:?}");
}

#[test]
fn multiple_issues() {
    let m = mutated(SNAPSHOT, &[("/manifest_version", json!("9.9")), ("/items/1/id", json!("item_1")), ("/items/0/image/path", json!("../x.png"))]);
    let issues = issues_of(&m);
    assert_eq!(issues.len(), 3, "{issues:?}");
    assert!(has_pointer(&issues, "/manifest_version"));
    assert!(has_pointer(&issues, "/items/1/id"));
    assert!(has_pointer(&issues, "/items/0/image/path"));
}
