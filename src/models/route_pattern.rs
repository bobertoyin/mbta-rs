//! Data model for MBTA route patterns.

use serde::{Deserialize, Serialize};

/// Attributes for a different variation of service that may be run within a single route, including when and how often they are operated.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct RoutePatternAttributes {
    /// The direction in which the trip is traveling: 0 or 1.
    pub direction_id: u8,
    /// User-facing description of where trips on the route pattern serve.
    pub name: String,
    /// Can be used to order the route patterns in a way which is ideal for presentation to customers.
    /// Route patterns with smaller sort_order values should be displayed before those with larger values.
    pub sort_order: u64,
    /// User-facing description of when the route pattern operate.
    pub time_desc: Option<String>,
    /// Explains how common the route pattern is. For the MBTA, this is within the context of the entire route.
    pub typicality: RoutePatternTypicality,
}

/// How common a route pattern is. For the MBTA, this is within the context of the entire route.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "u64")]
#[serde(into = "u64")]
pub enum RoutePatternTypicality {
    /// Not defined.
    Undefined,
    /// Typical. Pattern is common for the route. Most routes will have only one such pattern per direction.
    /// A few routes may have more than 1, such as the Red Line (with one branch to Ashmont and another to Braintree);
    /// routes with more than 2 are rare.
    Typical,
    /// Pattern is a deviation from the regular route.
    Deviation,
    /// Pattern represents a highly atypical pattern for the route, such as a special routing which only runs a handful of times per day.
    HighlyAtypical,
    /// Diversions from normal service, such as planned detours, bus shuttles, or snow routes.
    NormalServiceDiversion,
}

impl TryFrom<u64> for RoutePatternTypicality {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Undefined),
            1 => Ok(Self::Typical),
            2 => Ok(Self::Deviation),
            3 => Ok(Self::HighlyAtypical),
            4 => Ok(Self::NormalServiceDiversion),
            _ => Err(format!("invalid route pattern typicality: {}", value)),
        }
    }
}

impl Into<u64> for RoutePatternTypicality {
    fn into(self) -> u64 {
        match self {
            Self::Undefined => 0,
            Self::Typical => 1,
            Self::Deviation => 2,
            Self::HighlyAtypical => 3,
            Self::NormalServiceDiversion => 4,
        }
    }
}
