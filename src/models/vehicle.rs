//! Data models for MBTA vehicles.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::*;

/// Multiple vehicles.
pub type Vehicles = Vec<Vehicle>;

/// Current state of a vehicle on a trip.
pub type Vehicle = Resource<VehicleAttributes>;

/// Attributes for a vehicle.
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct VehicleAttributes {
    /// Time at which vehicle information was last updated.
    #[serde(with = "mbta_datetime_format")]
    pub updated_at: DateTime<FixedOffset>,
    /// Speed that the vehicle is traveling in meters per second.
    pub speed: Option<f64>,
    /// The degree of passenger occupancy for the vehicle.
    pub occupancy_status: Option<OccupancyStatus>,
    /// Longitude of the vehicle's current position. Degrees East, in the WGS-84 coordinate system.
    pub longitude: f64,
    /// Latitude of the vehicle's current position. Degrees North, in the WGS-84 coordinate system.
    pub latitude: f64,
    /// User visible label, such as the one of on the signage on the vehicle.
    pub label: String,
    /// Direction in which trip is traveling: 0 or 1.
    pub direction_id: Option<u8>,
    /// Index of current stop along trip.
    pub current_stop_sequence: Option<u64>,
    /// Status of vehicle relative to the stops.
    pub current_status: CurrentStatus,
    /// Bearing, in degrees, clockwise from True North, i.e., 0 is North and 90 is East.
    /// This can be the compass bearing, or the direction towards the next stop or intermediate location.
    pub bearing: u64,
}

/// Degree of passenger occupancy.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OccupancyStatus {
    /// Not crowded: the vehicle has a large percentage of seats available.
    ManySeatsAvailable,
    /// Some crowding: the vehicle has a small percentage of seats available.
    FewSeatsAvailable,
    /// Crowded: the vehicle is considered full by most measures, but may still be allowing passengers to board.
    Full,
}

/// Status relative to stops.
#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CurrentStatus {
    /// Just about to arrive at a stop.
    IncomingAt,
    /// Standing at a stop.
    StoppedAt,
    /// Departed the previous stop and is in transit.
    InTransitTo,
}
