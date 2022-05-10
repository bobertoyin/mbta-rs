//! Data model for MBTA route patterns.

use serde::{Deserialize, Serialize};

use super::*;

/// Multiple route patterns.
pub type RoutePatterns = Vec<RoutePattern>;

/// A different variation of service that may be run within a single route, including when and how often they are operated.
pub type RoutePattern = Resource<RoutePatternAttributes>;

/// Attributes for route pattern.
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
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(try_from = "u8")]
#[serde(into = "u8")]
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

impl TryFrom<u8> for RoutePatternTypicality {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl From<RoutePatternTypicality> for u8 {
    fn from(value: RoutePatternTypicality) -> Self {
        match value {
            RoutePatternTypicality::Undefined => 0,
            RoutePatternTypicality::Typical => 1,
            RoutePatternTypicality::Deviation => 2,
            RoutePatternTypicality::HighlyAtypical => 3,
            RoutePatternTypicality::NormalServiceDiversion => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case::zero(0, Ok(RoutePatternTypicality::Undefined))]
    #[case::one(1, Ok(RoutePatternTypicality::Typical))]
    #[case::two(2, Ok(RoutePatternTypicality::Deviation))]
    #[case::three(3, Ok(RoutePatternTypicality::HighlyAtypical))]
    #[case::four(4, Ok(RoutePatternTypicality::NormalServiceDiversion))]
    #[case::invalid(5, Err("invalid route pattern typicality: 5".into()))]
    fn test_route_pattern_typicality_try_from_u8(#[case] input: u8, #[case] expected: Result<RoutePatternTypicality, String>) {
        assert_eq!(RoutePatternTypicality::try_from(input), expected);
    }

    #[rstest]
    #[case::undefined(RoutePatternTypicality::Undefined, 0)]
    #[case::typical(RoutePatternTypicality::Typical, 1)]
    #[case::deviation(RoutePatternTypicality::Deviation, 2)]
    #[case::higly_atypical(RoutePatternTypicality::HighlyAtypical, 3)]
    #[case::normal_service_diversion(RoutePatternTypicality::NormalServiceDiversion, 4)]
    fn test_u8_from_route_pattern_typicality(#[case] input: RoutePatternTypicality, #[case] expected: u8) {
        assert_eq!(u8::from(input), expected);
    }
}
