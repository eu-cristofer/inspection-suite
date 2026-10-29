//! Chart specification and PNG rendering.

use serde::{Deserialize, Serialize};

use crate::error::ReportError;

/// Description of a chart to render.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartSpec {
    /// Chart title.
    pub title: String,
    /// Horizontal axis.
    pub x: Axis,
    /// Primary vertical axis.
    pub y: Axis,
    /// Optional secondary vertical axis.
    pub y2: Option<Axis>,
    /// Data series.
    pub series: Vec<Series>,
    /// Tolerance boxes drawn over the plot.
    pub boxes: Vec<ToleranceBox>,
}

/// One chart axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Axis {
    /// Axis label.
    pub label: String,
    /// Unit shown with the label.
    pub unit: String,
    /// Fixed minimum, or automatic when `None`.
    pub min: Option<f64>,
    /// Fixed maximum, or automatic when `None`.
    pub max: Option<f64>,
}

/// One data series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    /// Legend name.
    pub name: String,
    /// Drawing style.
    pub style: SeriesStyle,
    /// Data points as (x, y).
    pub points: Vec<(f64, f64)>,
    /// True to plot against the secondary axis.
    pub secondary_axis: bool,
}

/// How a series is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeriesStyle {
    /// Scattered points (test data).
    Points,
    /// Connected line (fitted curve).
    Line,
    /// Highlighted marker (for example the guarantee point).
    Marker,
}

/// A labelled rectangular tolerance region.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToleranceBox {
    /// Left edge.
    pub x_min: f64,
    /// Right edge.
    pub x_max: f64,
    /// Bottom edge.
    pub y_min: f64,
    /// Top edge.
    pub y_max: f64,
    /// Label drawn with the box.
    pub label: String,
}

/// Renders a chart to PNG bytes.
///
/// Stub in the frozen interface: always returns [`ReportError::NotImplemented`].
pub fn chart_png(spec: &ChartSpec, width_px: u32, height_px: u32) -> Result<Vec<u8>, ReportError> {
    let _ = (spec, width_px, height_px);
    Err(ReportError::NotImplemented)
}
