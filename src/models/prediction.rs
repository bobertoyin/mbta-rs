//! Data models for MBTA predictions.

use chrono::{offset::FixedOffset, DateTime};
use serde::{Deserialize, Serialize};

use super::*;

/// Attributes for the predicted arrival and departure time to/from a stop at a given sequence along a trip going a direction along a route.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct PredictionAttributes {
    /// The sequence the stop is arrived at during the trip.
    /// The stop sequence is monotonically increasing along the trip, but the stop_sequence along the trip are not necessarily consecutive.
    #[serde(default)]
    pub stop_sequence: Option<u64>,
    /// Status of the prediction.
    pub status: Option<String>,
    /// Direction in which trip is traveling: 0 or 1.
    pub direction_id: u8,
    /// When the vehicle is now predicted to depart. [None] if the last stop on the trip.
    #[serde(with = "optional_mbta_datetime_format")]
    pub departure_time: Option<DateTime<FixedOffset>>,
    /// When the vehicle is now predicted to arrive. [None] if the first stop on the trip.
    #[serde(with = "optional_mbta_datetime_format")]
    pub arrival_time: Option<DateTime<FixedOffset>>,
    /// How the predicted stop relates to the scheduled stops. [None] if the predicted stop was scheduled.
    pub schedule_relationship: Option<ScheduleRelationship>,
}

/// How a predicted stop relates to the scheduled stops.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScheduleRelationship {
    /// An extra trip that was added in addition to a running schedule, for example, to replace a broken vehicle or to respond to sudden passenger load.
    Added,
    /// A trip that existed in the schedule but was removed.
    Cancelled,
    /// No data is given for this stop. It indicates that there is no realtime information available.
    NoData,
    /// The stop was originally scheduled, but was skipped.
    Skipped,
    /// A trip that is running with no schedule associated to it.
    Unscheduled,
}
