//! Job definition: pump identity, rated point, test setup and fit options.

use serde::{Deserialize, Serialize};
use uom::si::f64::{AngularVelocity, Length, MassDensity, Power, Ratio, VolumeRate};

/// A complete test job description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Job {
    /// Pump identification.
    pub pump: PumpId,
    /// Guaranteed (rated) operating point.
    pub rated: RatedPoint,
    /// Instrumentation and test-bench setup.
    pub setup: TestSetup,
    /// Curve-fit options.
    pub fit: FitOptions,
}

/// Pump identification data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PumpId {
    /// Equipment tag.
    pub tag: String,
    /// Service description.
    pub service: Option<String>,
    /// Manufacturer name.
    pub manufacturer: Option<String>,
    /// Model designation.
    pub model: Option<String>,
    /// Serial number.
    pub serial: Option<String>,
    /// Number of stages.
    pub stages: u16,
}

/// Guaranteed operating point from the datasheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatedPoint {
    /// Rated flow.
    pub flow: VolumeRate,
    /// Rated total head.
    pub head: Length,
    /// Rated power.
    pub power: Power,
    /// Rated speed.
    pub speed: AngularVelocity,
    /// Guaranteed efficiency, if any.
    pub efficiency: Option<Ratio>,
    /// Required NPSH, if any.
    pub npshr: Option<Length>,
}

/// Test-bench geometry, fluid and power-measurement basis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSetup {
    /// Suction pipe inside diameter at the gauge.
    pub suction_id: Length,
    /// Discharge pipe inside diameter at the gauge.
    pub discharge_id: Length,
    /// Suction gauge elevation above the datum.
    pub z_suction: Length,
    /// Discharge gauge elevation above the datum.
    pub z_discharge: Length,
    /// Test fluid density.
    pub density: MassDensity,
    /// How power is measured.
    pub power_basis: PowerBasis,
}

/// Basis of the measured power.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PowerBasis {
    /// Power measured at the pump shaft.
    Shaft,
    /// Power measured at the motor input.
    MotorInput {
        /// Motor efficiency used to derive shaft power.
        motor_efficiency: Ratio,
    },
}

/// Polynomial degrees for the curve fits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FitOptions {
    /// Degree of the head curve.
    pub degree_head: u8,
    /// Degree of the power curve.
    pub degree_power: u8,
    /// Degree of the efficiency curve.
    pub degree_efficiency: u8,
}

impl Default for FitOptions {
    fn default() -> Self {
        Self { degree_head: 3, degree_power: 3, degree_efficiency: 2 }
    }
}
