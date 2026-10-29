//! Tolerance data model.
//!
//! Acceptance limits are DATA, entered by the owner from the licensed standard and loaded at run time. No standard number
//! appears in code.

use serde::{Deserialize, Serialize};
use uom::si::f64::Length;

use crate::error::PumpError;

/// Identifies the standard a tolerance set was transcribed from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StandardRef {
    /// Standard name.
    pub name: String,
    /// Edition identifier.
    pub edition: String,
    /// Table, person and date the values were taken from.
    pub source: String,
}

/// A set of acceptance criteria.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToleranceSet {
    /// Source standard.
    pub standard: StandardRef,
    /// True when the values are made-up examples, not real limits.
    pub example: bool,
    /// The criteria to evaluate.
    pub criteria: Vec<Criterion>,
}

/// One acceptance criterion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Criterion {
    /// Stable identifier.
    pub id: String,
    /// What the criterion checks.
    pub kind: CriterionKind,
    /// Clause or table reference in the standard.
    pub clause: String,
}

/// The kind of check a criterion performs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CriterionKind {
    /// Relative band around the guarantee at rated flow, selected by rated head.
    RelativeBand {
        /// Quantity checked.
        quantity: Quantity,
        /// Candidate bands; the first whose head condition matches applies.
        bands: Vec<Band>,
    },
    /// Relative upper limit only.
    MaxRelative {
        /// Quantity checked.
        quantity: Quantity,
        /// Upper limit in percent.
        upper_pct: f64,
    },
    /// The measured value must not exceed the guaranteed value.
    MaxAbsoluteVsGuarantee {
        /// Quantity checked.
        quantity: Quantity,
    },
    /// Maximum test-speed deviation from rated speed.
    SpeedDeviationMax {
        /// Maximum deviation in percent.
        max_pct: f64,
    },
    /// The head curve must rise continuously towards shutoff.
    HeadRisingToShutoff,
    /// Shutoff-head band relative to the guaranteed shutoff head.
    ShutoffBand {
        /// Lower limit in percent.
        lower_pct: f64,
        /// Upper limit in percent.
        upper_pct: f64,
    },
}

/// A tolerance band, optionally restricted by rated head.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Band {
    /// The band applies only if the rated head is at most this value; `None` means it always applies.
    pub applies_if_rated_head_max: Option<Length>,
    /// Lower limit in percent.
    pub lower_pct: f64,
    /// Upper limit in percent.
    pub upper_pct: f64,
}

/// A checked performance quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Quantity {
    /// Total head.
    Head,
    /// Power.
    Power,
    /// Efficiency.
    Efficiency,
    /// Required NPSH.
    Npshr,
}

/// Parses a tolerance file (TOML text) into a [`ToleranceSet`].
///
/// Stub in the frozen interface: always returns [`PumpError::NotImplemented`].
pub fn parse_tolerances(toml_text: &str) -> Result<ToleranceSet, PumpError> {
    let _ = toml_text;
    Err(PumpError::NotImplemented)
}
