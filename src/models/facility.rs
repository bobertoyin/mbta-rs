//! Data model for MBTA facilities.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Attributes for an amenity at a station stop such as an elevator, escalator, parking lot, or bike storage.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct FacilityAttributes {
    /// The type of facility.
    #[serde(rename = "type")]
    #[serde(default)]
    pub facility_type: Option<FacilityType>,
    /// The short name of the facility.
    #[serde(default)]
    pub short_name: Option<String>,
    /// Additional facility properties.
    #[serde(default)]
    pub properties: Vec<HashMap<String, Value>>, // would be nice to replace [Value] with some kind of union of String and Integer, based on the swagger docs
    /// The long name of the facility.
    #[serde(default)]
    pub long_name: Option<String>,
    /// The latitude of the facility. Degrees North, in the WGS-84 coordinate system.
    #[serde(default)]
    pub latitude: Option<f64>,
    /// The longitude of the facility. Degrees East, in the WGS-84 coordinate system.
    #[serde(default)]
    pub longitude: Option<f64>,
}

/// The types of facilities.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FacilityType {
    /// Bike storage.
    BikeStorage,
    /// Bridge plate.
    BridgePlate,
    /// Electric car chargers.
    ElectricCarChargers,
    /// Elevated sub-platform.
    ElevatedSubplatform,
    /// Elevator.
    Elevator,
    /// Escalator.
    Escalator,
    /// Fare media assistance facility.
    FareMediaAssistanceFacility,
    /// Fare media assistant.
    FareMediaAssistant,
    /// Fare vending machine.
    FareVendingMachine,
    /// Fare vending retailer.
    FareVendingRetailer,
    /// Fully elevated platform.
    FullyElevatedPlatform,
    /// Other.
    Other,
    /// Parking area.
    ParkingArea,
    /// Parking media.
    ParkingMedia,
    /// Pick drop.
    PickDrop,
    /// Portable boarding lift.
    PortableBoardingLift,
    /// Ramp.
    Ramp,
    /// Taxi stand.
    TaxiStand,
    /// Ticket window.
    TicketWindow,
}