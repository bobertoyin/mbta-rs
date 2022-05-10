//! Data models for MBTA trips.

use serde::{Deserialize, Serialize};

use super::*;

/// Multiple trips.
pub type Trips = Vec<Trip>;

/// The journey of a particular vehicle through a given set of stops.
pub type Trip = Resource<TripAttributes>;

/// Attributes for a trip.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct TripAttributes {
    /// Indicator of wheelchair accessibility.
    pub wheelchair_accessible: WheelchairAccessible,
    /// The text that appears in schedules and sign boards to identify the trip to passengers.
    /// For example, to identify train numbers for commuter rail trips.
    pub name: String,
    /// The text that appears on a sign that identifies the trip’s destination to passengers.
    pub headsign: String,
    /// Direction in which trip is traveling: 0 or 1.
    pub direction_id: u8,
    /// ID used to group sequential trips with the same vehicle for a given service id.
    pub block_id: String,
    /// Indicator of whether or not bikes are allowed on this trip.
    pub bikes_allowed: BikesAllowed,
}

/// Whether or not a bike is allowed.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum BikesAllowed {
    /// No information.
    NoInfo,
    /// Can accomodate at least one bicycle.
    Allowed,
    /// No bicycles are allowed.
    NotAllowed,
}

impl TryFrom<u8> for BikesAllowed {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoInfo),
            1 => Ok(Self::Allowed),
            2 => Ok(Self::NotAllowed),
            _ => Err(format!("invalid bikes allowed value: {}", value)),
        }
    }
}

impl From<BikesAllowed> for u8 {
    fn from(value: BikesAllowed) -> Self {
        match value {
            BikesAllowed::NoInfo => 0,
            BikesAllowed::Allowed => 1,
            BikesAllowed::NotAllowed => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::zero(0, Ok(BikesAllowed::NoInfo))]
    #[case::one(1, Ok(BikesAllowed::Allowed))]
    #[case::two(2, Ok(BikesAllowed::NotAllowed))]
    #[case::invalid(3, Err("invalid bikes allowed value: 3".into()))]
    fn test_bikes_allowed_try_from_u8(#[case] input: u8, #[case] expected: Result<BikesAllowed, String>) {
        assert_eq!(BikesAllowed::try_from(input), expected);
    }

    #[rstest]
    #[case::no_info(BikesAllowed::NoInfo, 0)]
    #[case::allowed(BikesAllowed::Allowed, 1)]
    #[case::not_allowed(BikesAllowed::NotAllowed, 2)]
    fn test_u8_from_bikes_allowed(#[case] input: BikesAllowed, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }
}
