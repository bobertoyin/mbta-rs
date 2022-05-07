<!-- PROJECT LOGO -->
<div align="center">
<img src="https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png" alt="MBTA Logo" width="80" height="80">
<h2>MBTA-RS</h2>
<p>A simple Rust client for interacting with the <a href="https://www.mbta.com/developers/v3-api">Massachusetts Bay Transport Authority's V3 API</a>*</p>
<small>*This project is not affiliated with any official development work from the MBTA</small>
</div>

<!-- ABOUT THE PROJECT -->
## About

The MBTA V3 API is described as: 

> A fast, flexible, standards-based API for schedules, arrival predictions, vehicle locations, and service alerts.

This project provides a simple synchronous client and "opinionated" data models to easily consume data from the API within your Rust code.

> Opinionated? In what way is this crate "opinionated"?

The models that are returned from the client are "opinionated" due to the highly dynamic nature of the MBTA's V3 API: there are a plethora of query parameters that can change the number of fields per JSON object and even mix-and-match different model schemas! This crate opts to lock down any interface to these query params in favor of keeping a few that don't affect the return schema (currently page limits and page offsets) and decides what information is relevant to return. It is then up to the user to implement the remaining query parameter features, e.g. filtering the results or building the relationships between models yourself. TLDR; this crate defines all of the relevant data model fields, no more and no less.

### Built With

- [ureq](https://crates.io/crates/ureq) as the underlying HTTP client
- [Serde](https://crates.io/crates/serde) and [Serde JSON](https://crates.io/crates/serde_json) for data deserialization/serialization
- [Chrono](https://crates.io/crates/chrono) for handling datetime data

> Why provide a synchronous client rather than an asynchronous one?

1. I didn't want this crate to be tied down to a specific `async` runtime
2. I wanted to use the `ureq` crate for its simple API and small size, but it only provides a synchronous API

> Why not auto-generate a client, given that the V3 API utilizes OpenAPI/Swagger?

1. I'm not very familiar with any of the code generation tools available
2. I'd personally prefer to have a handcrafted client with some sharper data definitions than one that is auto-generated
3. There aren't *too* many API endpoints as of now, so manual maintenance shouldn't be too much of an issue once kicked off
4. I like subjecting myself to unnecessary and Sisyphean tasks

<!-- USAGE -->
## Usage

> It is highly recommended to have the [API Swagger docs](https://api-v3.mbta.com/docs/swagger/index.html) handy, as it generally contains more detailed and thorough documentaiton for model fields than this crate's API.

In your `Cargo.toml` file:
```toml
[dependencies]
mbta-rs = "*"
```

<!-- CONTRIBUTE -->
## Contribute

See `CONTRIBUTE.md` to get started!

<!-- OTHER ACKNOWLEDGEMENTS -->
## Other Acknowledgements

- [Matt Vertescher's MBTA V3 API](https://github.com/mvertescher/mbta-v3-swagger-api-client-rs) for already existing! 
- [Matt Boulanger's Mattermost API client](https://crates.io/crates/mattermost_api) for some level of client design inspiration
- [Othneil Drew's Best README Template](https://github.com/othneildrew/Best-README-Template) for templating the README layout
