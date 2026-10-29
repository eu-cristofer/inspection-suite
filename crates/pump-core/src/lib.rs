//! API 610 shop performance-test calculations and acceptance verdicts. Pure: no I/O, no UI.
//!
//! The public types are a frozen contract (decision D-008): changes need a decision entry in `docs/decisions.md` and a heads-up
//! to the pump-core and report-kit lanes.

mod curve;
mod error;
mod job;
mod point;
mod report;
mod tolerance;
mod verdict;

pub use curve::{Curve, Curves, RatedReadout};
pub use error::PumpError;
pub use job::{FitOptions, Job, PowerBasis, PumpId, RatedPoint, TestSetup};
pub use point::{RatedSpeedPoint, ReducedPoint, TestPoint};
pub use report::{AcceptanceReport, evaluate};
pub use tolerance::{Band, Criterion, CriterionKind, Quantity, StandardRef, ToleranceSet, parse_tolerances};
pub use verdict::{Limit, Outcome, Verdict, VerdictStatus};
