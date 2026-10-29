#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::{Manifest, PackageKind};
use report_kit::*;
use serde::Serialize;
use serde::de::DeserializeOwned;

fn sample_doc() -> ReportDoc {
    ReportDoc {
        title: "Report".into(),
        subtitle: Some("Sub".into()),
        meta: vec![("Job".into(), "Job-001".into())],
        sections: vec![Section {
            heading: "Results".into(),
            blocks: vec![
                Block::Paragraph("text".into()),
                Block::Table(Table { header: vec!["a".into(), "b".into()], rows: vec![vec!["1".into(), "2".into()]], caption: Some("cap".into()) }),
                Block::Image(Figure { png: vec![137, 80, 78, 71], caption: "fig".into(), width_mm: 120.5 }),
                Block::Verdict { label: "Head".into(), pass: true, detail: "within band".into() },
            ],
        }],
    }
}

fn axis(label: &str) -> Axis {
    Axis { label: label.into(), unit: "u".into(), min: Some(0.0), max: None }
}

fn sample_chart() -> ChartSpec {
    let series = |name: &str, style| Series { name: name.into(), style, points: vec![(0.0, 1.0), (1.0, 2.5)], secondary_axis: false };
    ChartSpec {
        title: "Chart".into(),
        x: axis("Flow"),
        y: axis("Head"),
        y2: Some(axis("Power")),
        series: vec![series("pts", SeriesStyle::Points), series("line", SeriesStyle::Line), series("mark", SeriesStyle::Marker)],
        boxes: vec![ToleranceBox { x_min: 0.0, x_max: 1.0, y_min: 1.0, y_max: 2.0, label: "box".into() }],
    }
}

fn roundtrip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(value, &back);
}

#[test]
fn doc_roundtrips() {
    roundtrip(&sample_doc());
}

#[test]
fn chart_roundtrips() {
    roundtrip(&sample_chart());
}

#[test]
fn render_stubs_are_not_implemented() {
    assert_eq!(chart_png(&sample_chart(), 800, 600), Err(ReportError::NotImplemented));
    assert_eq!(to_docx(&sample_doc()), Err(ReportError::NotImplemented));
    assert_eq!(to_pdf(&sample_doc()), Err(ReportError::NotImplemented));
}

#[test]
fn package_stub_is_not_implemented() {
    let target = std::env::temp_dir().join("report-kit-contract-never-written");
    let manifest = Manifest::new(PackageKind::PumpTest, "1.0");
    assert_eq!(write_package(&target, &manifest, &[("a.txt".to_string(), vec![1])], false), Err(ReportError::NotImplemented));
    assert!(!target.exists());
}
