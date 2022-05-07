//! Data models for shared/common data.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

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
