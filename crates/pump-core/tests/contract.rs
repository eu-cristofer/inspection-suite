#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // test helpers outside #[test] fns

use evidence_format::ToolInfo;
use pump_core::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use uom::si::f64::{AngularVelocity, Length, MassDensity, Power, Pressure, Ratio, VolumeRate};

fn flow(m3h: f64) -> VolumeRate {
    VolumeRate::new::<uom::si::volume_rate::cubic_meter_per_hour>(m3h)
}
fn meters(v: f64) -> Length {
    Length::new::<uom::si::length::meter>(v)
}
fn kw(v: f64) -> Power {
    Power::new::<uom::si::power::kilowatt>(v)
}
fn rpm(v: f64) -> AngularVelocity {
    AngularVelocity::new::<uom::si::angular_velocity::revolution_per_minute>(v)
}
fn percent(v: f64) -> Ratio {
    Ratio::new::<uom::si::ratio::percent>(v)
}
fn bar(v: f64) -> Pressure {
    Pressure::new::<uom::si::pressure::bar>(v)
}

fn sample_job() -> Job {
    Job {
        pump: PumpId {
            tag: "P-101".into(),
            service: Some("Example service".into()),
            manufacturer: None,
            model: Some("Model-X".into()),
            serial: None,
            stages: 2,
        },
        rated: RatedPoint {
            flow: flow(250.0),
            head: meters(85.0),
            power: kw(78.0),
            speed: rpm(2980.0),
            efficiency: Some(percent(76.0)),
            npshr: Some(meters(4.0)),
        },
        setup: TestSetup {
            suction_id: meters(0.2),
            discharge_id: meters(0.15),
            z_suction: meters(0.5),
            z_discharge: meters(1.0),
            density: MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(998.0),
            power_basis: PowerBasis::MotorInput { motor_efficiency: percent(94.0) },
        },
        fit: FitOptions::default(),
    }
}

fn sample_point() -> TestPoint {
    TestPoint { flow: flow(250.0), p_suction: bar(1.0), p_discharge: bar(9.5), speed: rpm(2975.0), power: kw(80.0) }
}

/// Made-up values only (`example = true`); real limits never appear in this public repo.
fn sample_tolerances() -> ToleranceSet {
    ToleranceSet {
        standard: StandardRef { name: "EXAMPLE".into(), edition: "0".into(), source: "made-up values".into() },
        example: true,
        criteria: vec![
            Criterion {
                id: "c1".into(),
                clause: "x.1".into(),
                kind: CriterionKind::RelativeBand {
                    quantity: Quantity::Head,
                    bands: vec![
                        Band { applies_if_rated_head_max: Some(meters(100.0)), lower_pct: -3.0, upper_pct: 5.0 },
                        Band { applies_if_rated_head_max: None, lower_pct: -2.0, upper_pct: 4.0 },
                    ],
                },
            },
            Criterion { id: "c2".into(), clause: "x.2".into(), kind: CriterionKind::MaxRelative { quantity: Quantity::Power, upper_pct: 4.0 } },
            Criterion { id: "c3".into(), clause: "x.3".into(), kind: CriterionKind::MaxAbsoluteVsGuarantee { quantity: Quantity::Npshr } },
            Criterion { id: "c4".into(), clause: "x.4".into(), kind: CriterionKind::SpeedDeviationMax { max_pct: 3.0 } },
            Criterion { id: "c5".into(), clause: "x.5".into(), kind: CriterionKind::HeadRisingToShutoff },
            Criterion { id: "c6".into(), clause: "x.6".into(), kind: CriterionKind::ShutoffBand { lower_pct: -10.0, upper_pct: 10.0 } },
        ],
    }
}

fn sample_curve(degree: u8) -> Curve {
    Curve { degree, coefficients: vec![90.0, -1.5, -400.0], r_squared: 0.999, rms_residual: 0.05 }
}

fn sample_report() -> AcceptanceReport {
    let reduced = ReducedPoint {
        measured: sample_point(),
        head: meters(84.0),
        hydraulic_power: kw(57.0),
        shaft_power: kw(75.0),
        efficiency: percent(76.0),
        speed_deviation: percent(-0.17),
        at_rated_speed: RatedSpeedPoint { flow: flow(250.5), head: meters(84.2), shaft_power: kw(75.2) },
    };
    AcceptanceReport {
        job: sample_job(),
        points: vec![reduced],
        curves: Curves { head: sample_curve(3), power: sample_curve(3), efficiency: sample_curve(2) },
        at_rated: RatedReadout { head: meters(84.5), power: kw(76.0), efficiency: percent(75.5), shutoff_head: Some(meters(95.0)) },
        verdicts: vec![
            Verdict {
                criterion_id: "c1".into(),
                clause: "x.1".into(),
                limit: Limit { lower: Some(-3.0), upper: Some(5.0), unit: "%".into() },
                measured: Some(84.5),
                deviation_pct: Some(-0.6),
                status: VerdictStatus::Pass,
            },
            Verdict {
                criterion_id: "c2".into(),
                clause: "x.2".into(),
                limit: Limit { lower: None, upper: Some(4.0), unit: "%".into() },
                measured: Some(76.0),
                deviation_pct: Some(5.0),
                status: VerdictStatus::Fail,
            },
            Verdict {
                criterion_id: "c3".into(),
                clause: "x.3".into(),
                limit: Limit { lower: None, upper: None, unit: "m".into() },
                measured: None,
                deviation_pct: None,
                status: VerdictStatus::NotEvaluated { reason: "no NPSH data".into() },
            },
        ],
        overall: Outcome::Fail,
        standard: StandardRef { name: "EXAMPLE".into(), edition: "0".into(), source: "made-up values".into() },
        tool: ToolInfo { name: "pump-core".into(), version: "0.0.0".into(), commit: None },
    }
}

fn roundtrip<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(value, &back);
}

#[test]
fn job_roundtrips() {
    roundtrip(&sample_job());
    roundtrip(&PowerBasis::Shaft);
}

#[test]
fn points_roundtrip() {
    roundtrip(&sample_point());
}

#[test]
fn tolerances_roundtrip() {
    roundtrip(&sample_tolerances());
}

#[test]
fn report_roundtrips() {
    roundtrip(&sample_report());
}

#[test]
fn default_fit_options() {
    assert_eq!(FitOptions::default(), FitOptions { degree_head: 3, degree_power: 3, degree_efficiency: 2 });
}

#[test]
fn stubs_are_not_implemented() {
    assert_eq!(evaluate(&sample_job(), &[sample_point()], &sample_tolerances()), Err(PumpError::NotImplemented));
    assert_eq!(parse_tolerances(""), Err(PumpError::NotImplemented));
}
