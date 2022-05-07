//! Data model for MBTA schedules.

use chrono::{offset::FixedOffset, DateTime};
use serde::{Deserialize, Serialize};

use super::*;

/// Attributes for an arrival drop-off time and departure pick-up time to/from a stop at a given sequence along a trip
/// going in a direction on a route when the trip is following a service to determine when it is active.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ScheduleAttributes {
    /// Whether time points for a schedule are exact or estimates.
    pub timepoint: ScheduleTimepoint,
    /// The sequence the stop is arrived at during the trip.
    /// The stop sequence is monotonically increasing along the trip, but the stop_sequence along the trip are not necessarily consecutive.
    pub stop_sequence: u64,
    /// Text identifying destination of the trip, overriding trip-level headsign if present.
    pub stop_headsign: Option<String>,
    /// How the vehicle departs from the stop.
    pub pickup_type: VehiclePresence,
    /// How the vehicle arrives at the stop.
    pub drop_off_type: VehiclePresence,
    /// Direction in which the trip is traveling: 0 or 1.
    pub direction_id: u8,
    /// Time when the trip departs the given stop.
    #[serde(with = "mbta_date_format")]
    pub departure_time: DateTime<FixedOffset>,
    /// Time when the trip arrives at the given stop.
    #[serde(with = "mbta_date_format")]
    pub arrival_time: DateTime<FixedOffset>,
}

/// Whether time points are exact or estimates.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(from = "bool")]
#[serde(into = "bool")]
pub enum ScheduleTimepoint {
    /// Exact time points.
    Exact,
    /// Estimated/interpolated time points.
    Estimates,
}

impl From<bool> for ScheduleTimepoint {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Exact,
            false => Self::Estimates,
        }
    }
}

impl Into<bool> for ScheduleTimepoint {
    fn into(self) -> bool {
        match self {
            Self::Exact => true,
            Self::Estimates => false,
        }
    }
}

/// How a vehicle is scheduled to be present at a stop.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "u64")]
#[serde(into = "u64")]
pub enum VehiclePresence {
    /// Regularly scheduled pick-up/drop-off.
    RegularlyScheduled,
    /// Not available for pick-up/drop-off.
    NotAvailable,
    /// Must phone agency to arrange pick-up/drop-off.
    MustPhoneAgency,
    /// Must coordinate with driver to arrange pick-up/drop-off.
    MustCoordinateWithDriver,
}

impl TryFrom<u64> for VehiclePresence {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RegularlyScheduled),
            1 => Ok(Self::NotAvailable),
            2 => Ok(Self::MustPhoneAgency),
            3 => Ok(Self::MustCoordinateWithDriver),
            _ => Err(format!("invalid vehicle presence: {}", value)),
        }
    }
}

impl Into<u64> for VehiclePresence {
    fn into(self) -> u64 {
        match self {
            Self::RegularlyScheduled => 0,
            Self::NotAvailable => 1,
            Self::MustPhoneAgency => 2,
            Self::MustCoordinateWithDriver => 3,
        }
    }
}
