#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png"
)]
#![doc = include_str!("../README.md")]

pub mod client;
pub use client::*;
pub mod error;
pub use error::*;
pub mod models;
pub use models::*;
