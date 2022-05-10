//! Data models for MBTA live facilities.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::*;

/// Multiple live facilities.
pub type LiveFacilities = Vec<LiveFacility>;

/// Live data about a given facility.
pub type LiveFacility = Resource<LiveFacilityAttributes>;

/// Attributes for a live facility.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct LiveFacilityAttributes {
    /// Time of last update.
    #[serde(with = "mbta_datetime_format")]
    pub updated_at: DateTime<FixedOffset>,
    /// Properties.
    pub properties: Vec<LiveFacilityProperty>,
}

/// Properties for a live facility.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct LiveFacilityProperty {
    /// The name of the property.
    pub name: String,
    /// The value of the property.
    pub value: Value,
}
