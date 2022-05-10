//! Data models for MBTA services.

use chrono::{Date, FixedOffset};
use serde::{Deserialize, Serialize};

use super::*;

/// Multiple services.
pub type Services = Vec<Service>;

/// A set of dates on which trips run.
pub type Service = Resource<ServiceAttributes>;

/// Attributes for service.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ServiceAttributes {
    /// Days of the week.
    pub valid_days: Vec<Day>,
    /// Earliest date which is valid for this service.
    #[serde(with = "mbta_date_format")]
    pub start_date: Date<FixedOffset>,
    /// Describes how well this schedule represents typical service for the listed schedule type.
    pub schedule_typicality: ScheduleTypicality,
    /// Description of the schedule type the service can be applied.
    pub schedule_type: Option<String>,
    /// Description of when the service is in effect.
    pub schedule_name: Option<String>,
    /// Extra information about exceptional dates (e.g. holiday name).
    pub removed_dates_notes: Vec<Option<String>>,
    /// Exceptional dates when the service is not valid.
    #[serde(with = "vec_mbta_date_format")]
    pub removed_dates: Vec<Date<FixedOffset>>,
    /// Earliest date which is a part of the rating (season) which contains this service.
    #[serde(with = "optional_mbta_date_format")]
    pub rating_start_date: Option<Date<FixedOffset>>,
    /// Latest date which is a part of the rating (season) which contains this service.
    #[serde(with = "optional_mbta_date_format")]
    pub rating_end_date: Option<Date<FixedOffset>>,
    /// Human-readable description of the rating (season), as it should appear on public-facing websites and applications.
    pub rating_description: Option<String>,
    /// Latest date which is valid for this service.
    #[serde(with = "mbta_date_format")]
    pub end_date: Date<FixedOffset>,
    /// Human-readable description of the service, as it should appear on public-facing websites and applications.
    pub description: Option<String>,
    /// Extra information about additional dates (e.g. holiday name).
    pub added_dates_notes: Vec<Option<String>>,
    /// Additional dates when the service is valid.
    #[serde(with = "vec_mbta_date_format")]
    pub added_dates: Vec<Date<FixedOffset>>,
}

/// Represents how well a schedule represents typical service for a listed schedule type.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum ScheduleTypicality {
    /// Not defined.
    Undefined,
    /// Typical service with perhaps minor modifications.
    Typical,
    /// Extra service supplements typical schedules.
    Extra,
    /// Reduced holiday service is provided by typical Saturday or Sunday schedule.
    Reduced,
    /// Major changes in service due to a planned disruption, such as construction.
    Disrupted,
    /// Major reductions in service for weather events or other atypical situations.
    Atypical,
}

impl TryFrom<u8> for ScheduleTypicality {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Undefined),
            1 => Ok(Self::Typical),
            2 => Ok(Self::Extra),
            3 => Ok(Self::Reduced),
            4 => Ok(Self::Disrupted),
            5 => Ok(Self::Atypical),
            _ => Err(format!("invalid schedule typicality value: {}", value)),
        }
    }
}

impl From<ScheduleTypicality> for u8 {
    fn from(value: ScheduleTypicality) -> Self {
        match value {
            ScheduleTypicality::Undefined => 0,
            ScheduleTypicality::Typical => 1,
            ScheduleTypicality::Extra => 2,
            ScheduleTypicality::Reduced => 3,
            ScheduleTypicality::Disrupted => 4,
            ScheduleTypicality::Atypical => 5,
        }
    }
}

/// Represents a day of the week.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum Day {
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Sunday.
    Sunday,
}

impl TryFrom<u8> for Day {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            7 => Ok(Self::Sunday),
            _ => Err(format!("invalid day value: {}", value)),
        }
    }
}

impl From<Day> for u8 {
    fn from(value: Day) -> Self {
        match value {
            Day::Monday => 1,
            Day::Tuesday => 2,
            Day::Wednesday => 3,
            Day::Thursday => 4,
            Day::Friday => 5,
            Day::Saturday => 6,
            Day::Sunday => 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::zero(0, Ok(ScheduleTypicality::Undefined))]
    #[case::one(1, Ok(ScheduleTypicality::Typical))]
    #[case::two(2, Ok(ScheduleTypicality::Extra))]
    #[case::three(3, Ok(ScheduleTypicality::Reduced))]
    #[case::four(4, Ok(ScheduleTypicality::Disrupted))]
    #[case::five(5, Ok(ScheduleTypicality::Atypical))]
    #[case::invalid(6, Err("invalid schedule typicality value: 6".into()))]
    fn test_schedule_typicality_try_from_u8(#[case] input: u8, #[case] expected: Result<ScheduleTypicality, String>) {
        assert_eq!(ScheduleTypicality::try_from(input), expected);
    }

    #[rstest]
    #[case::undefined(ScheduleTypicality::Undefined, 0)]
    #[case::typical(ScheduleTypicality::Typical, 1)]
    #[case::extra(ScheduleTypicality::Extra, 2)]
    #[case::reduced(ScheduleTypicality::Reduced, 3)]
    #[case::disrupted(ScheduleTypicality::Disrupted, 4)]
    #[case::atypical(ScheduleTypicality::Atypical, 5)]
    fn test_u8_from_schedule_typicality(#[case] input: ScheduleTypicality, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[rstest]
    #[case::one(1, Ok(Day::Monday))]
    #[case::two(2, Ok(Day::Tuesday))]
    #[case::three(3, Ok(Day::Wednesday))]
    #[case::four(4, Ok(Day::Thursday))]
    #[case::five(5, Ok(Day::Friday))]
    #[case::six(6, Ok(Day::Saturday))]
    #[case::seven(7, Ok(Day::Sunday))]
    #[case::invalid(8, Err("invalid day value: 8".into()))]
    fn test_day_try_from_u8(#[case] input: u8, #[case] expected: Result<Day, String>) {
        assert_eq!(Day::try_from(input), expected);
    }

    #[rstest]
    #[case::monday(Day::Monday, 1)]
    #[case::tuesday(Day::Tuesday, 2)]
    #[case::wednesday(Day::Wednesday, 3)]
    #[case::thursday(Day::Thursday, 4)]
    #[case::friday(Day::Friday, 5)]
    #[case::saturday(Day::Saturday, 6)]
    #[case::sunday(Day::Sunday, 7)]
    fn test_u8_from_day(#[case] input: Day, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }
}
