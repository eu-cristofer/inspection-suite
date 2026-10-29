//! Per-criterion verdicts.

use serde::{Deserialize, Serialize};

/// The acceptance limits applied, in display units.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    /// Lower limit, if any.
    pub lower: Option<f64>,
    /// Upper limit, if any.
    pub upper: Option<f64>,
    /// Unit of the limits.
    pub unit: String,
}

/// Status of one criterion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerdictStatus {
    /// Criterion satisfied.
    Pass,
    /// Criterion violated.
    Fail,
    /// Criterion could not be evaluated; never counted as a pass.
    NotEvaluated {
        /// Why evaluation was skipped.
        reason: String,
    },
}

/// Result of evaluating one criterion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verdict {
    /// Criterion identifier.
    pub criterion_id: String,
    /// Clause reference.
    pub clause: String,
    /// Limits applied.
    pub limit: Limit,
    /// Measured value, if available.
    pub measured: Option<f64>,
    /// Deviation from the guarantee in percent, if applicable.
    pub deviation_pct: Option<f64>,
    /// Pass, fail or not evaluated.
    pub status: VerdictStatus,
}

/// Overall acceptance outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// Every applicable criterion passed.
    Pass,
    /// At least one criterion failed or could not be evaluated.
    Fail,
}
