//! Data model for MBTA lines.

use serde::{Deserialize, Serialize};

use super::*;

/// Multiple lines.
pub type Lines = Vec<Line>;

/// A combination of routes.
pub type Line = Resource<LineAttributes>;

/// Attributes for a line.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct LineAttributes {
    /// A color that corresponds to a line.
    /// The color must be provided as a six-character hexadecimal number, for example, 00FFFF.
    pub color: String,
    /// A legible color to use for text drawn against a background of line_color.
    /// The color must be provided as a six-character hexadecimal number, for example, FFD700.
    pub text_color: String,
    /// Lines sort in ascending order.
    pub sort_order: u64,
    /// Short, public-facing name for the group of routes represented in this line.
    pub short_name: String,
    /// Lengthier, public-facing name for the group of routes represented in this line.
    pub long_name: String,
}
