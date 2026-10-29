#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::{PackageKind, from_json, to_json};
use serde_json::Value;

const SNAPSHOT: &str = include_str!("fixtures/snapshot_v1_1.json");
const PUMP: &str = include_str!("fixtures/pump_test_v1_0.json");

fn round_trip(text: &str) {
    let manifest = from_json(text).expect("parse");
    let out = to_json(&manifest).expect("serialize");
    assert!(out.ends_with('\n'));
    let original: Value = serde_json::from_str(text).expect("orig value");
    let output: Value = serde_json::from_str(&out).expect("out value");
    assert_eq!(original, output);
}

#[test]
fn snapshot_round_trips_without_loss() {
    round_trip(SNAPSHOT);
}

#[test]
fn pump_round_trips_without_loss() {
    round_trip(PUMP);
}

#[test]
fn snapshot_envelope_fields() {
    let m = from_json(SNAPSHOT).expect("parse");
    assert_eq!(m.package_kind, PackageKind::SnapshotExport);
    for key in ["source_pdf", "coordinate_space", "rendering"] {
        assert!(m.extra.contains_key(key), "missing {key}");
    }
}
