//! Data models for shared/common data.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Serialization and deserialization for the MBTA datetime format.
pub mod mbta_date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Datetime string format.
    pub const FORMAT: &'static str = "%FT%T%:z";

    /// Serialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `date` - the datetime
    /// * `serializer` - the serializer
    pub fn serialize<S: Serializer>(
        datetime: &DateTime<FixedOffset>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", datetime.format(FORMAT)))
    }

    /// Attempt to deserialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<FixedOffset>, D::Error> {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

/// Serialization and deserialization for an optional MBTA datetime format.
pub mod optional_mbta_date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    use super::mbta_date_format::{serialize as date_serialize, FORMAT};

    /// Serialize an optional MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `datetime` - the optional datetime
    /// * `serializer` - the serializer
    pub fn serialize<S>(
        datetime: &Option<DateTime<FixedOffset>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(d) => date_serialize(d, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Attempt to deserialize an optional MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let date =
                    DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

/// MBTA V3 API response object.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Response<D> {
    /// Data payload of the response.
    pub data: D,
    /// JSON API version.
    pub jsonapi: APIVersion,
    /// Links to different pages of the endpoint.
    #[serde(default)]
    pub links: Option<Links>,
}

/// Version of the JSON API.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct APIVersion {
    /// Version as a string.
    pub version: String,
}

/// Page limit and offset numbers to the first, next, and last pages of the endpoint.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Links {
    /// HTTP link to the first page of the endpoint.
    pub first: String,
    /// HTTP link to the next page of the endpoint.
    pub next: String,
    /// HTTP link to the last page of the endpoint.
    pub last: String,
}

/// Some MBTA resource, bundling common metadata with the actual model attributes.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Resource<Attribute> {
    /// The JSON API resource type.
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The JSON API resource id.
    pub id: String,
    /// Related endpoint links. *This field could use some more documentation.*
    #[serde(default)]
    pub links: HashMap<String, String>,
    /// Model attributes.
    pub attributes: Attribute,
}

/// The type of transportation a route supports.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "i64")]
#[serde(into = "i64")]
pub enum RouteType {
    /// Light rail transportation.
    LightRail,
    /// Heavy rail transportation.
    HeavyRail,
    /// Commuter rail transportation.
    CommuterRail,
    /// Bus transportation.
    Bus,
    /// Ferry transportation.
    Ferry,
}

impl TryFrom<i64> for RouteType {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LightRail),
            1 => Ok(Self::HeavyRail),
            2 => Ok(Self::CommuterRail),
            3 => Ok(Self::Bus),
            4 => Ok(Self::Ferry),
            _ => Err(format!("invalid route type: {}", value)),
        }
    }
}

impl Into<i64> for RouteType {
    fn into(self) -> i64 {
        match self {
            Self::LightRail => 0,
            Self::HeavyRail => 1,
            Self::CommuterRail => 2,
            Self::Bus => 3,
            Self::Ferry => 4,
        }
    }
}

#[cfg(test)]
mod tests_route_type {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::light_rail(0, Ok(RouteType::LightRail))]
    #[case::heavy_rail(1, Ok(RouteType::HeavyRail))]
    #[case::commuter_rail(2, Ok(RouteType::CommuterRail))]
    #[case::bus(3, Ok(RouteType::Bus))]
    #[case::ferry(4, Ok(RouteType::Ferry))]
    #[case::invalid_route_type(5, Err(format!("invalid route type: {}", 5)))]
    fn test_route_type_try_from_ref_str(
        #[case] input: i64,
        #[case] expected: Result<RouteType, String>,
    ) {
        // Arrange

        // Act
        let actual = RouteType::try_from(input);

        // Assert
        assert_eq!(actual, expected)
    }

    #[rstest]
    #[case::light_rail(RouteType::LightRail, 0)]
    #[case::heavy_rail(RouteType::HeavyRail, 1)]
    #[case::commuter_rail(RouteType::CommuterRail, 2)]
    #[case::bus(RouteType::Bus, 3)]
    #[case::ferry(RouteType::Ferry, 4)]
    fn test_route_type_into_ref_str(#[case] input: RouteType, #[case] expected: i64) {
        // Arrange

        // Act
        let actual: i64 = input.into();

        // Assert
        assert_eq!(actual, expected);
    }
}