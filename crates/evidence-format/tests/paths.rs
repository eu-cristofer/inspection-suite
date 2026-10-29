#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::{FormatError, normalize_package_relative_path as norm};

fn is_path_err(r: Result<String, FormatError>) -> bool {
    matches!(r, Err(FormatError::Path(_)))
}

#[test]
fn rejects_empty() {
    assert!(is_path_err(norm("")));
}

#[test]
fn rejects_backslash() {
    assert!(is_path_err(norm("a\\b")));
}

#[test]
fn rejects_absolute() {
    assert!(is_path_err(norm("/a/b")));
}

#[test]
fn rejects_parent() {
    assert!(is_path_err(norm("a/../b")));
    assert!(is_path_err(norm("../a")));
}

#[test]
fn rejects_dot_only() {
    assert!(is_path_err(norm(".")));
}

#[test]
fn normalizes_interior_dot() {
    assert_eq!(norm("a/./b").expect("ok"), "a/b");
}

#[test]
fn accepts_plain() {
    assert_eq!(norm("evidence/item_1.png").expect("ok"), "evidence/item_1.png");
}
