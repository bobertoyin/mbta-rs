#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png"
)]
#![doc = include_str!("../README.md")]

pub mod client;
pub use client::{Client, BASE_URL};
pub mod error;
pub use error::*;
pub mod models;
pub use models::*;
