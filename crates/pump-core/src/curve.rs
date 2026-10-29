//! Fitted performance curves.

use serde::{Deserialize, Serialize};
use uom::si::f64::{Length, Power, Ratio};

/// A fitted polynomial curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Curve {
    /// Polynomial degree.
    pub degree: u8,
    /// Coefficients in SI, ascending powers of flow in m³/s.
    pub coefficients: Vec<f64>,
    /// Coefficient of determination.
    pub r_squared: f64,
    /// Root-mean-square residual, in SI units of the fitted quantity.
    pub rms_residual: f64,
}

/// The three fitted performance curves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Curves {
    /// Head versus flow.
    pub head: Curve,
    /// Shaft power versus flow.
    pub power: Curve,
    /// Efficiency versus flow.
    pub efficiency: Curve,
}

/// Curve values read at the rated flow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatedReadout {
    /// Head at rated flow.
    pub head: Length,
    /// Power at rated flow.
    pub power: Power,
    /// Efficiency at rated flow.
    pub efficiency: Ratio,
    /// Shutoff head, only when a zero-flow point was tested.
    pub shutoff_head: Option<Length>,
}
