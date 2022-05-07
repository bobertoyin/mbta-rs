//! Data models for shared/common data.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Serialization and deserialization for the MBTA datetime format.
pub mod mbta_date_format {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Datetime string format.
    pub const FORMAT: &str = "%FT%T%:z";

    /// Serialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `date` - the datetime
    /// * `serializer` - the serializer
    pub fn serialize<S: Serializer>(datetime: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&format!("{}", datetime.format(FORMAT)))
    }

    /// Attempt to deserialize an MBTA datetime.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - the deserializer
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error> {
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
    pub fn serialize<S>(datetime: &Option<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
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
                let date = DateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

/// MBTA V3 API response objects.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Response<D> {
    /// A successful response object.
    Success(ResponseSuccess<D>),
    /// An error response.
    Error(ResponseError),
}

impl<D> From<ResponseSuccess<D>> for Response<D> {
    fn from(value: ResponseSuccess<D>) -> Self {
        Response::Success(value)
    }
}

impl<D> From<ResponseError> for Response<D> {
    fn from(value: ResponseError) -> Self {
        Response::Error(value)
    }
}

/// MBTA V3 API successful response object.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ResponseSuccess<D> {
    /// Data payload of the response.
    pub data: D,
    /// JSON API version.
    pub jsonapi: APIVersion,
    /// Links to different pages of the endpoint.
    #[serde(default)]
    pub links: Option<Links>,
}

/// MBTA V3 API error response object.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ResponseError {
    /// Errors.
    pub errors: Vec<HashMap<String, Value>>,
    /// JSON API version.
    pub jsonapi: APIVersion,
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

/// The type of transportation a route supports.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "u64")]
#[serde(into = "u64")]
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

impl TryFrom<u64> for RouteType {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
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

impl From<RouteType> for u64 {
    fn from(value: RouteType) -> u64 {
        match value {
            RouteType::LightRail => 0,
            RouteType::HeavyRail => 1,
            RouteType::CommuterRail => 2,
            RouteType::Bus => 3,
            RouteType::Ferry => 4,
        }
    }
}
