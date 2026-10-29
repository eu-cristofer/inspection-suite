//! Test points as measured and as reduced.

use serde::{Deserialize, Serialize};
use uom::si::f64::{AngularVelocity, Length, Power, Pressure, Ratio, VolumeRate};

/// One measured test point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestPoint {
    /// Measured flow.
    pub flow: VolumeRate,
    /// Suction gauge pressure.
    pub p_suction: Pressure,
    /// Discharge gauge pressure.
    pub p_discharge: Pressure,
    /// Measured speed.
    pub speed: AngularVelocity,
    /// Measured power, on the basis given by [`crate::PowerBasis`].
    pub power: Power,
}

/// A test point after data reduction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReducedPoint {
    /// The original measurement.
    pub measured: TestPoint,
    /// Total head.
    pub head: Length,
    /// Hydraulic power.
    pub hydraulic_power: Power,
    /// Shaft power.
    pub shaft_power: Power,
    /// Pump efficiency.
    pub efficiency: Ratio,
    /// Relative deviation of test speed from rated speed.
    pub speed_deviation: Ratio,
    /// The point converted to rated speed by the affinity laws.
    pub at_rated_speed: RatedSpeedPoint,
}

/// A point converted to rated speed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatedSpeedPoint {
    /// Flow at rated speed.
    pub flow: VolumeRate,
    /// Head at rated speed.
    pub head: Length,
    /// Shaft power at rated speed.
    pub shaft_power: Power,
}
