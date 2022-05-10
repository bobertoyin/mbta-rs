//! Data model for MBTA schedules.

use chrono::{offset::FixedOffset, DateTime};
use serde::{Deserialize, Serialize};

use super::*;

/// Multiple schedules.
pub type Schedules = Vec<Schedule>;

/// An arrival drop-off time and departure pick-up time to/from a stop at a given sequence along a trip
/// going in a direction on a route when the trip is following a service to determine when it is active.
pub type Schedule = Resource<ScheduleAttributes>;

/// Attributes for a schedule.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ScheduleAttributes {
    /// Whether time points for a schedule are exact or estimates.
    pub timepoint: ScheduleTimepoint,
    /// The sequence the stop is arrived at during the trip.
    /// The stop sequence is monotonically increasing along the trip, but the stop_sequence along the trip are not necessarily consecutive.
    pub stop_sequence: Option<u64>,
    /// Text identifying destination of the trip, overriding trip-level headsign if present.
    pub stop_headsign: Option<String>,
    /// How the vehicle departs from the stop.
    pub pickup_type: VehiclePresence,
    /// How the vehicle arrives at the stop.
    pub drop_off_type: VehiclePresence,
    /// Direction in which the trip is traveling: 0 or 1.
    pub direction_id: u8,
    /// Time when the trip departs the given stop.
    #[serde(with = "optional_mbta_datetime_format")]
    pub departure_time: Option<DateTime<FixedOffset>>,
    /// Time when the trip arrives at the given stop.
    #[serde(with = "optional_mbta_datetime_format")]
    pub arrival_time: Option<DateTime<FixedOffset>>,
}

/// Whether time points are exact or estimates.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
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

impl From<ScheduleTimepoint> for bool {
    fn from(value: ScheduleTimepoint) -> Self {
        match value {
            ScheduleTimepoint::Exact => true,
            ScheduleTimepoint::Estimates => false,
        }
    }
}

/// How a vehicle is scheduled to be present at a stop.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
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

impl TryFrom<u8> for VehiclePresence {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RegularlyScheduled),
            1 => Ok(Self::NotAvailable),
            2 => Ok(Self::MustPhoneAgency),
            3 => Ok(Self::MustCoordinateWithDriver),
            _ => Err(format!("invalid vehicle presence value: {}", value)),
        }
    }
}

impl From<VehiclePresence> for u8 {
    fn from(value: VehiclePresence) -> Self {
        match value {
            VehiclePresence::RegularlyScheduled => 0,
            VehiclePresence::NotAvailable => 1,
            VehiclePresence::MustPhoneAgency => 2,
            VehiclePresence::MustCoordinateWithDriver => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::true_value(true, ScheduleTimepoint::Exact)]
    #[case::false_value(false, ScheduleTimepoint::Estimates)]
    fn test_schedule_timepoint_from_bool(#[case] input: bool, #[case] expected: ScheduleTimepoint) {
        assert_eq!(ScheduleTimepoint::from(input), expected);
    }

    #[rstest]
    #[case::exact(ScheduleTimepoint::Exact, true)]
    #[case::estimates(ScheduleTimepoint::Estimates, false)]
    fn test_bool_from_schedule_timepoint(#[case] input: ScheduleTimepoint, #[case] expected: bool) {
        assert_eq!(bool::from(input), expected);
    }

    #[rstest]
    #[case::zero(0, Ok(VehiclePresence::RegularlyScheduled))]
    #[case::one(1, Ok(VehiclePresence::NotAvailable))]
    #[case::two(2, Ok(VehiclePresence::MustPhoneAgency))]
    #[case::three(3, Ok(VehiclePresence::MustCoordinateWithDriver))]
    #[case::invalid(4, Err("invalid vehicle presence value: 4".into()))]
    fn test_vehicle_presence_try_from_u8(#[case] input: u8, #[case] expected: Result<VehiclePresence, String>) {
        assert_eq!(VehiclePresence::try_from(input), expected);
    }

    #[rstest]
    #[case::regularly_scheduled(VehiclePresence::RegularlyScheduled, 0)]
    #[case::not_available(VehiclePresence::NotAvailable, 1)]
    #[case::must_phone_agency(VehiclePresence::MustPhoneAgency, 2)]
    #[case::must_coordinate_with_driver(VehiclePresence::MustCoordinateWithDriver, 3)]
    fn test_u8_from_vehicle_presence(#[case] input: VehiclePresence, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }
}
