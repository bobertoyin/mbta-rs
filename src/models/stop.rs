//! Data models for MBTA stops.

use serde::{Deserialize, Serialize};

use super::*;

/// Multiple stops.
pub type Stops = Vec<Stop>;

/// A physical location where transit can pick-up or drop-off passengers.
pub type Stop = Resource<StopAttributes>;

/// Attributes for a stop.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct StopAttributes {
    /// Whether there are any vehicles with wheelchair boarding or paths to stops that are wheelchair accessible.
    pub wheelchair_boarding: WheelchairAccessible,
    /// The type of transportation used at the stop.
    pub vehicle_type: Option<RouteType>,
    /// A textual description of the platform or track.
    pub platform_name: Option<String>,
    /// A short code representing the platform/track (like a number or letter).
    pub platform_code: Option<String>,
    /// The street on which the stop is located.
    pub on_street: Option<String>,
    /// Name of a stop or station in the local and tourist vernacular.
    pub name: String,
    /// The municipality in which the stop is located.
    pub municipality: Option<String>,
    /// Longitude of the stop or station. Degrees East, in the WGS-84 coordinate system.
    pub longitude: f64,
    /// Latitude of the stop or station. Degrees North, in the WGS-84 coordinate system.
    pub latitude: f64,
    /// Description of the stop.
    pub description: Option<String>,
    /// The cross street at which the stop is located.
    pub at_street: Option<String>,
    /// A street address for the station.
    pub address: Option<String>,
    /// The type of the stop.
    pub location_type: LocationType,
}

/// The type of stop.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum LocationType {
    /// A location where passengers board or disembark from a transit vehicle.
    Stop,
    /// A physical structure or area that contains one or more stops.
    Station,
    /// A location where passengers can enter or exit a station from the street.
    /// The stop entry must also specify a parent_station value referencing the stop ID of the parent station for the entrance.
    EntranceExit,
    /// A location within a station, not matching any other location type.
    GenericNode,
}

impl TryFrom<u8> for LocationType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Stop),
            1 => Ok(Self::Station),
            2 => Ok(Self::EntranceExit),
            3 => Ok(Self::GenericNode),
            _ => Err(format!("invalid location type value: {}", value)),
        }
    }
}

impl From<LocationType> for u8 {
    fn from(value: LocationType) -> Self {
        match value {
            LocationType::Stop => 0,
            LocationType::Station => 1,
            LocationType::EntranceExit => 2,
            LocationType::GenericNode => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::zero(0, Ok(LocationType::Stop))]
    #[case::one(1, Ok(LocationType::Station))]
    #[case::two(2, Ok(LocationType::EntranceExit))]
    #[case::three(3, Ok(LocationType::GenericNode))]
    #[case::four(4, Err("invalid location type value: 4".into()))]
    fn test_location_type_try_from_u8(#[case] input: u8, #[case] expected: Result<LocationType, String>) {
        assert_eq!(LocationType::try_from(input), expected);
    }

    #[rstest]
    #[case::stop(LocationType::Stop, 0)]
    #[case::station(LocationType::Station, 1)]
    #[case::entrance_exit(LocationType::EntranceExit, 2)]
    #[case::generic_node(LocationType::GenericNode, 3)]
    fn test_u8_from_location_type(#[case] input: LocationType, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }
}
