#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/6/64/MBTA.svg/1200px-MBTA.svg.png",
    html_favicon_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/6/64/MBTA.svg/1200px-MBTA.svg.png"
)]
#![doc = include_str!("../README.md")]

pub mod client;
pub use client::*;
pub mod error;
pub use error::*;
pub mod models;
pub use models::*;
