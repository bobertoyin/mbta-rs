//! Data models for MBTA alerts.

use chrono::{offset::FixedOffset, DateTime};
use serde::{Deserialize, Serialize};

use super::*;

/// Multiple alerts.
pub type Alerts = Vec<Alert>;

/// An active or upcoming system alert.
pub type Alert = Resource<AlertAttributes>;

/// Attributes for an alert.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct AlertAttributes {
    /// A URL for extra details, such as outline construction or maintenance plans.
    pub url: Option<String>,
    /// Date/Time alert created.
    #[serde(with = "mbta_datetime_format")]
    pub created_at: DateTime<FixedOffset>,
    /// Date/Time alert last updated.
    #[serde(with = "mbta_datetime_format")]
    pub updated_at: DateTime<FixedOffset>,
    /// Summarizes when an alert is in effect.
    pub timeframe: Option<String>,
    /// Header description for an alert.
    pub header: String,
    /// A shortened version of `header`.
    pub short_header: String,
    /// How severe the alert is, from 0 (least severe) to 10 (most severe).
    pub severity: u8,
    /// Summarizes the service and the impact to that service.
    pub service_effect: String,
    /// Identifies whether an alert is a new or old, in effect or upcoming.
    pub lifecycle: Lifecycle,
    /// The effect of this problem on the affected entity.
    pub effect: Effect,
    /// This plain-text string will be formatted as the body of the alert (or shown on an explicit “expand” request by the user).
    /// The information in the description should add to the information of the header.
    pub description: Option<String>,
    /// What is causing the alert.
    pub cause: Cause,
    /// Set if alert is meant to be displayed prominently, such as the top of every page.
    pub banner: Option<String>,
    /// Active periods for an alert.
    pub active_period: Vec<ActivePeriod>,
    /// Entities affected by an alert.
    pub informed_entity: Vec<InformedEntity>,
}

/// Start and end dates for an active alert.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
pub struct ActivePeriod {
    /// Start date for an active alert.
    #[serde(with = "mbta_datetime_format")]
    pub start: DateTime<FixedOffset>,
    /// End date for an active alert.
    #[serde(with = "optional_mbta_datetime_format")]
    pub end: Option<DateTime<FixedOffset>>,
}

/// An entity affected by an alert. At least one of the fields other than activities will be non-null.
/// The affected entity is the intersection of these fields, not the union: if stop and route both have values, the alert does not affect the entire route.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct InformedEntity {
    /// ID of the affected trip.
    pub trip: Option<String>,
    /// ID of the affected stop.
    pub stop: Option<String>,
    /// Type of the affected route.
    pub route_type: Option<RouteType>,
    /// ID of the affected route.
    pub route: Option<String>,
    /// ID of the affected facility.
    pub facility: Option<String>,
    /// Direction ID of the affected trip.
    pub direction_id: Option<u8>,
    /// Activities affected by the alert.
    pub activities: Vec<Activity>,
}

/// Whether an alert is a new or old, in effect or upcoming.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Lifecycle {
    /// A new alert.
    New,
    /// An ongoing alert.
    Ongoing,
    /// An ongoing and upcoming alert.
    OngoingUpcoming,
    /// An upcoming alert.
    Upcoming,
}

/// The effect of a problem on an affected entity.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Effect {
    /// Access issue.
    AccessIssue,
    /// Additional service.
    AdditionalService,
    /// Amber alert.
    AmberAlert,
    /// Bike issue.
    BikeIssue,
    /// Cancellation.
    Cancellation,
    /// Delay.
    Delay,
    /// Detour.
    Detour,
    /// Dock closure.
    DockClosure,
    /// Dock issue.
    DockIssue,
    /// Elevator closure.
    ElevatorClosure,
    /// Escalator closure.
    EscalatorClosure,
    /// Extra service.
    ExtraService,
    /// Facility issue.
    FacilityIssue,
    /// Modified service.
    ModifiedService,
    /// No service.
    NoService,
    /// Other effect.
    OtherEffect,
    /// Parking closure.
    ParkingClosure,
    /// Parking issue.
    ParkingIssue,
    /// Policy change.
    PolicyChange,
    /// Schedule change.
    ScheduleChange,
    /// Service change.
    ServiceChange,
    /// Shuttle.
    Shuttle,
    /// Snow route.
    SnowRoute,
    /// Station closure.
    StationClosure,
    /// Station issue.
    StationIssue,
    /// Stop closure.
    StopClosure,
    /// Stop move.
    StopMove,
    /// Stop moved.
    StopMoved,
    /// Summary.
    Summary,
    /// Suspension.
    Suspension,
    /// Track change.
    TrackChange,
    /// Unknown effect.
    UnknownEffect,
}

/// What is causing an alert.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Cause {
    /// A general accident.
    Accident,
    /// Amtrak.
    Amtrak,
    /// An earlier mechanical problem.
    AnEarlierMechanicalProblem,
    /// An earlier signal problem.
    AnEarlierSignalProblem,
    /// Automobiles impeding service.
    AutosImpedingService,
    /// Coast guard restriction.
    CoastGuardRestriction,
    /// Congestion.
    Congestion,
    /// Construction.
    Construction,
    /// Crossing malfunction.
    CrossingMalfunction,
    /// Demonstration.
    Demonstration,
    /// Disabled bus.
    DisabledBus,
    /// Disabled train.
    DisabledTrain,
    /// Drawbridge being raised.
    DrawbridgeBeingRaised,
    /// Electrical work.
    ElectricalWork,
    /// Fire.
    Fire,
    /// Fog.
    Fog,
    /// Freight train interference.
    FreightTrainInterference,
    /// Hazmat condition.
    HazmatCondition,
    /// Heavy ridership.
    HeavyRidership,
    /// High winds.
    HighWinds,
    /// Holiday.
    Holiday,
    /// Hurricane.
    Hurricane,
    /// Ice in harbor.
    IceInHarbor,
    /// Maintenance.
    Maintenance,
    /// Mechanical problem.
    MechanicalProblem,
    /// Medical emergency.
    MedicalEmergency,
    /// Parade.
    Parade,
    /// Police action.
    PoliceAction,
    /// Power problem.
    PowerProblem,
    /// Severe weather.
    SevereWeather,
    /// Signal problem.
    SignalProblem,
    /// Slippery rail.
    SlipperyRail,
    /// Snow.
    Snow,
    /// Special event.
    SpecialEvent,
    /// Speed restriction.
    SpeedRestriction,
    /// Switch problem.
    SwitchProblem,
    /// Tie replacement.
    TieReplacement,
    /// Track problem.
    TrackProblem,
    /// Track work.
    TrackWork,
    /// Traffic.
    Traffic,
    /// Unruly passenger.
    UnrulyPassenger,
    /// Unknown cause.
    UnknownCause,
    /// Weather.
    Weather,
}

/// An activity affected by an alert.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Activity {
    /// Boarding a vehicle.
    Board,
    /// Bringing a bike.
    BringingBike,
    /// Exiting.
    Exit,
    /// Parking a car.
    ParkCar,
    /// Riding a vehicle.
    Ride,
    /// Storing a bike.
    StoreBike,
    /// Using an escalator.
    UsingEscalator,
    /// Using a wheelchair.
    UsingWheelchair,
}
