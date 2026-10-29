#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::{PackageKind, json_schema};

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots");
const FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots/pump_test_schema.json");

#[test]
fn pump_test_schema_matches_snapshot() {
    let actual = json_schema(PackageKind::PumpTest).expect("schema");
    if std::env::var("UPDATE_SNAPSHOTS").as_deref() == Ok("1") {
        std::fs::create_dir_all(DIR).expect("create dir");
        std::fs::write(FILE, &actual).expect("write snapshot");
        return;
    }
    let expected = std::fs::read_to_string(FILE).unwrap_or_else(|e| panic!("cannot read {FILE}: {e}; run with UPDATE_SNAPSHOTS=1 to generate it"));
    assert_eq!(actual, expected, "schema changed; rerun with UPDATE_SNAPSHOTS=1 if intended");
}
