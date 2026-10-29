#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::{FormatError, Manifest, PackageKind, ToolInfo};
use serde_json::json;

#[test]
fn set_then_get_round_trips() {
    let mut m = Manifest::new(PackageKind::PumpTest, "1.0");
    assert_eq!(m.tool().expect("ok"), None);
    let tool = ToolInfo { name: "pump-cli".into(), version: "0.1.0".into(), commit: Some("abc123".into()) };
    m.set_tool(&tool).expect("set");
    assert_eq!(m.tool().expect("ok"), Some(tool));
}

#[test]
fn wrong_type_is_tool_error() {
    let mut m = Manifest::new(PackageKind::PumpTest, "1.0");
    m.extra.insert("tool".into(), json!("not an object"));
    assert!(matches!(m.tool(), Err(FormatError::Tool(_))));
}
