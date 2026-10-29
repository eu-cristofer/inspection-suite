//! Domain-neutral report document model.

use serde::{Deserialize, Serialize};

/// A complete report document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportDoc {
    /// Document title.
    pub title: String,
    /// Optional subtitle.
    pub subtitle: Option<String>,
    /// Key/value metadata shown in the header block.
    pub meta: Vec<(String, String)>,
    /// Body sections in order.
    pub sections: Vec<Section>,
}

/// A titled group of blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    /// Section heading.
    pub heading: String,
    /// Content blocks in order.
    pub blocks: Vec<Block>,
}

/// One content block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Block {
    /// A paragraph of plain text.
    Paragraph(String),
    /// A table.
    Table(Table),
    /// An embedded image.
    Image(Figure),
    /// A pass/fail verdict line, rendered with text as well as colour.
    Verdict {
        /// Short label.
        label: String,
        /// True for pass, false for fail.
        pass: bool,
        /// Explanatory detail.
        detail: String,
    },
}

/// A table of text cells.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    /// Column headers.
    pub header: Vec<String>,
    /// Row cells.
    pub rows: Vec<Vec<String>>,
    /// Optional caption.
    pub caption: Option<String>,
}

/// An embedded PNG figure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Figure {
    /// PNG bytes.
    pub png: Vec<u8>,
    /// Figure caption.
    pub caption: String,
    /// Rendered width in millimetres.
    pub width_mm: f32,
}
