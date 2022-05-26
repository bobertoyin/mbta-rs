<!-- HEADER -->
<div align="center">
<image src="https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png" alt="MBTA Logo" width="80" height="80">
<h2><b>MBTA-RS</b></h2>
<image src="https://img.shields.io/crates/v/mbta-rs?style=flat-square">
<image src ="https://img.shields.io/crates/l/mbta-rs?style=flat-square">
<image src="https://img.shields.io/docsrs/mbta-rs?style=flat-square">
<image src="https://img.shields.io/github/workflow/status/bobertoyin/mbta-rs/CI?style=flat-square">
<hr>
<p>A simple Rust client for interacting with the <a href="https://www.mbta.com/developers/v3-api">Massachusetts Bay Transport Authority's V3 API</a>*</p>
<small>*This project is not affiliated with any official development work from the MBTA</small>
</div>

<!-- ABOUT THE PROJECT -->
## About

The MBTA V3 API is described as: 

> A fast, flexible, standards-based API for schedules, arrival predictions, vehicle locations, and service alerts.

This project provides a simple synchronous client and data models to easily consume data from the API within your Rust code.

### Built With

- [ureq](https://crates.io/crates/ureq) as the underlying HTTP client
- [Serde](https://crates.io/crates/serde) and [Serde JSON](https://crates.io/crates/serde_json) for data deserialization/serialization
- [Chrono](https://crates.io/crates/chrono) for handling datetime data

> Why provide a synchronous client rather than an asynchronous one?

1. I didn't want this crate to be tied down to a specific `async` runtime
2. I wanted to use the `ureq` crate for its simple API and small size, and it only provides a synchronous client

> Why not auto-generate a client, given that the OpenAPI/Swagger client code-generators exists?

1. I'm not very familiar with any of the code generation tools available
2. I'd personally prefer to have a handcrafted client with some sharper data definitions than one that is auto-generated
3. There aren't *too* many API endpoints as of now, so manual maintenance shouldn't be too much of an issue once kicked off
4. I like subjecting myself to unnecessary and Sisyphean tasks

<!-- USAGE -->
## Usage

> It is highly recommended to have the [API Swagger docs](https://api-v3.mbta.com/docs/swagger/index.html) handy, as it generally contains more detailed and thorough documentation for model field than what is provided here.

In your `Cargo.toml` file:
```toml
[dependencies]
mbta-rs = "*"

# if you need to manipulate/further inspect certain fields
chrono = "*"
serde_json = "*"
```

Simple example usage:
```rust
use std::env;
use mbta_rs::Client;

let client = match env::var("MBTA_TOKEN") {
    Ok(token) => Client::with_key(token),
    Err(_) => Client::without_key()
};

let query_params = [
    ("page[limit]", "3")
];

let alerts_response = client.alerts(&query_params);
if let Ok(response) = alerts_response {
    for alert in response.data {
        println!("MBTA alert: {}", alert.attributes.header);
    }
}
```

## Map Feature

This library comes with an optional module for plotting location-related data models (stops, vehicles, shapes, etc.) onto a simple tile map.

In your `Cargo.toml` file:
```toml
[dependencies]
mbta-rs = { version = "*", features = ["map"] }

# necessary to create the map itself
staticmap = "*"
```

Simple example usage:
```rust
use std::{collections::HashMap, env};
use staticmap::StaticMapBuilder;
use mbta_rs::{Client, map::{Plottable, PlotStyle}};

let client = match env::var("MBTA_TOKEN") {
    Ok(token) => Client::with_key(token),
    Err(_) => Client::without_key()
};

let routes = client.routes(&[("filter[type]", "0,1")]).expect("failed to get routes");
let mut map = StaticMapBuilder::new()
    .width(1000)
    .height(1000)
    .zoom(12)
    .lat_center(42.326768)
    .lon_center(-71.100099)
    .build()
    .expect("failed to build map");

for route in routes.data {
    let query_params = [("filter[route]", &route.id)];
    let shapes = client
        .shapes(&query_params)
        .expect("failed to get shapes");
    for shape in shapes.data {
        shape
            .plot(&mut map, true, PlotStyle::new((route.attributes.color.clone(), 3.0), Some(("#FFFFFF".into(), 1.0))))
            .expect("failed to plot shape");
    }
    let stops = client
        .stops(&query_params)
        .expect("failed to get stops");
    for stop in stops.data {
        stop.plot(
            &mut map,
            true,
            PlotStyle::new((route.attributes.color.clone(), 3.0), Some(("#FFFFFF".into(), 1.0))),
        )
        .expect("failed to plot stop");
    }
}

// save to file...
```

<!-- CONTRIBUTE -->
## Contribute

See [CONTRIBUTE.md](https://github.com/bobertoyin/mbta-rs/blob/main/CONTRIBUTE.md) to get started!

<!-- OTHER ACKNOWLEDGEMENTS -->
## Other Acknowledgements

- [Matt Vertescher's MBTA V3 API](https://github.com/mvertescher/mbta-v3-swagger-api-client-rs) for already existing! 
- [Matt Boulanger's Mattermost API client](https://crates.io/crates/mattermost_api) for some of the client design inspiration
- [Othneil Drew's Best README Template](https://github.com/othneildrew/Best-README-Template) for templating the `README` and `CONTRIBUTE` layouts
