//! Data model for MBTA routes.

use serde::{Deserialize, Serialize};

use super::*;

/// Attributes for a path a vehicle travels during service.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct RouteAttributes {
    /// The type of route.
    #[serde(rename = "type")]
    pub route_type: RouteType,
    /// The short name of the facility.
    pub short_name: String,
    /// The long name of the facility.
    pub long_name: String,
    /// A color that corresponds to a line.
    /// The color must be provided as a six-character hexadecimal number, for example, 00FFFF.
    pub color: String,
    /// A legible color to use for text drawn against a background of line_color.
    /// The color must be provided as a six-character hexadecimal number, for example, FFD700.
    pub text_color: String,
    /// Routes sort in ascending order.
    pub sort_order: u64,
    /// Specifies the fare type of the route, which can differ from the service category.
    pub fare_class: String,
    /// The names of direction ids for this route in ascending ordering starting at 0 for the first index.
    #[serde(default)]
    pub direction_names: Option<Vec<String>>,
    /// The destinations for direction ids for this route in ascending ordering starting at 0 for the first index.
    #[serde(default)]
    pub direction_destinations: Option<Vec<String>>,
    /// Details about stops, schedule, and/or service.
    pub description: String,
}