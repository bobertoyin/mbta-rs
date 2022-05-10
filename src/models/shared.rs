//! Data models for shared/common data.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub first: Option<String>,
    /// HTTP link to the next page of the endpoint.
    #[serde(default)]
    pub next: Option<String>,
    /// HTTP link to the last page of the endpoint.
    #[serde(default)]
    pub last: Option<String>,
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
    pub links: Option<HashMap<String, String>>,
    /// Model attributes.
    pub attributes: Attribute,
    /// Relationships to other data models.
    #[serde(default)]
    pub relationships: Option<HashMap<String, Relationships>>,
}

/// A model's relationships to other data models.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Relationships {
    /// Another model that is related to this data model.
    pub data: Option<RelationshipAtom>,
}

/// Atomic data for relationships between data models.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct RelationshipAtom {
    /// The type of the related model.
    #[serde(rename = "type")]
    pub relationship_type: String,
    /// The ID of the related model.
    pub id: String,
}

/// The type of transportation something can support.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
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

impl TryFrom<u8> for RouteType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LightRail),
            1 => Ok(Self::HeavyRail),
            2 => Ok(Self::CommuterRail),
            3 => Ok(Self::Bus),
            4 => Ok(Self::Ferry),
            _ => Err(format!("invalid route type value: {}", value)),
        }
    }
}

impl From<RouteType> for u8 {
    fn from(value: RouteType) -> u8 {
        match value {
            RouteType::LightRail => 0,
            RouteType::HeavyRail => 1,
            RouteType::CommuterRail => 2,
            RouteType::Bus => 3,
            RouteType::Ferry => 4,
        }
    }
}

/// Whether something is wheelchair accessible.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
pub enum WheelchairAccessible {
    /// No information.
    NoInfo,
    /// Accessible.
    Accessible,
    /// Inaccessible.
    Inaccessible,
}

impl TryFrom<u8> for WheelchairAccessible {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoInfo),
            1 => Ok(Self::Accessible),
            2 => Ok(Self::Inaccessible),
            _ => Err(format!("invalid wheelchair accessibility value: {}", value)),
        }
    }
}

impl From<WheelchairAccessible> for u8 {
    fn from(value: WheelchairAccessible) -> Self {
        match value {
            WheelchairAccessible::NoInfo => 0,
            WheelchairAccessible::Accessible => 1,
            WheelchairAccessible::Inaccessible => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::zero(0, Ok(RouteType::LightRail))]
    #[case::one(1, Ok(RouteType::HeavyRail))]
    #[case::two(2, Ok(RouteType::CommuterRail))]
    #[case::three(3, Ok(RouteType::Bus))]
    #[case::four(4, Ok(RouteType::Ferry))]
    #[case::invalid(5, Err("invalid route type value: 5".into()))]
    fn test_route_type_try_from_u8(#[case] input: u8, #[case] expected: Result<RouteType, String>) {
        assert_eq!(RouteType::try_from(input), expected);
    }

    #[rstest]
    #[case::light_rail(RouteType::LightRail, 0)]
    #[case::heavy_rail(RouteType::HeavyRail, 1)]
    #[case::commuter_rail(RouteType::CommuterRail, 2)]
    #[case::bus(RouteType::Bus, 3)]
    #[case::ferry(RouteType::Ferry, 4)]
    fn test_u8_from_route_type(#[case] input: RouteType, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }

    #[rstest]
    #[case::zero(0, Ok(WheelchairAccessible::NoInfo))]
    #[case::one(1, Ok(WheelchairAccessible::Accessible))]
    #[case::two(2, Ok(WheelchairAccessible::Inaccessible))]
    #[case::invalid(3, Err("invalid wheelchair accessibility value: 3".into()))]
    fn test_wheelchair_accessible_try_from_u8(#[case] input: u8, #[case] expected: Result<WheelchairAccessible, String>) {
        assert_eq!(WheelchairAccessible::try_from(input), expected);
    }

    #[rstest]
    #[case::no_info(WheelchairAccessible::NoInfo, 0)]
    #[case::accessible(WheelchairAccessible::Accessible, 1)]
    #[case::inaccessible(WheelchairAccessible::Inaccessible, 2)]
    fn test_u8_from_wheelchair_accessible(#[case] input: WheelchairAccessible, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }
}
