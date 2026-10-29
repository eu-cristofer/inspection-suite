//! Acceptance report and the evaluation entry point.

use serde::{Deserialize, Serialize};

use crate::curve::{Curves, RatedReadout};
use crate::error::PumpError;
use crate::job::Job;
use crate::point::{ReducedPoint, TestPoint};
use crate::tolerance::{StandardRef, ToleranceSet};
use crate::verdict::{Outcome, Verdict};

/// Full result of evaluating a job against a tolerance set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcceptanceReport {
    /// The evaluated job.
    pub job: Job,
    /// Reduced test points.
    pub points: Vec<ReducedPoint>,
    /// Fitted curves.
    pub curves: Curves,
    /// Curve values at rated flow.
    pub at_rated: RatedReadout,
    /// Per-criterion verdicts.
    pub verdicts: Vec<Verdict>,
    /// Overall outcome.
    pub overall: Outcome,
    /// Standard the tolerances came from.
    pub standard: StandardRef,
    /// Tool that produced the report.
    pub tool: evidence_format::ToolInfo,
}

/// Reduces the test points, fits the curves and evaluates every criterion.
///
/// Stub in the frozen interface: always returns [`PumpError::NotImplemented`].
pub fn evaluate(job: &Job, points: &[TestPoint], tolerances: &ToleranceSet) -> Result<AcceptanceReport, PumpError> {
    let _ = (job, points, tolerances);
    Err(PumpError::NotImplemented)
}
