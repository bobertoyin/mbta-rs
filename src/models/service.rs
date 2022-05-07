//! Data models for MBTA services.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::*;

/// Attributes for a set of dates on which trips run.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ServiceAttributes {
    /// Days of the week.
    pub valid_days: Vec<Day>,
    /// Earliest date which is valid for this service.
    #[serde(with = "mbta_date_format")]
    pub start_date: NaiveDate,
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
    pub removed_dates: Vec<NaiveDate>,
    /// Earliest date which is a part of the rating (season) which contains this service.
    #[serde(with = "optional_mbta_date_format")]
    pub rating_start_date: Option<NaiveDate>,
    /// Latest date which is a part of the rating (season) which contains this service.
    #[serde(with = "optional_mbta_date_format")]
    pub rating_end_date: Option<NaiveDate>,
    /// Human-readable description of the rating (season), as it should appear on public-facing websites and applications.
    pub rating_description: Option<String>,
    /// Latest date which is valid for this service.
    #[serde(with = "mbta_date_format")]
    pub end_date: NaiveDate,
    /// Human-readable description of the service, as it should appear on public-facing websites and applications.
    pub description: Option<String>,
    /// Extra information about additional dates (e.g. holiday name).
    pub added_dates_notes: Vec<Option<String>>,
    /// Additional dates when the service is valid.
    #[serde(with = "vec_mbta_date_format")]
    pub added_dates: Vec<NaiveDate>,
}

/// Represents how well a schedule represents typical service for a listed schedule type.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
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
    Distrupted,
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
            4 => Ok(Self::Distrupted),
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
            ScheduleTypicality::Distrupted => 4,
            ScheduleTypicality::Atypical => 5,
        }
    }
}

/// Represents a day of the week.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
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
