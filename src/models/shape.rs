//! Data models for MBTA shapes.

use serde::{Deserialize, Serialize};

use super::*;

/// Multiple shapes.
pub type Shapes = Vec<Shape>;

/// A sequence of geographic points representing a path vehicles will travel on a trip.
pub type Shape = Resource<ShapeAttributes>;

/// Attributes for a shape.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ShapeAttributes {
    /// The sequence of points in Encoded Polyline Algorithm Format.
    pub polyline: String,
}
