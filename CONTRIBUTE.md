<!-- PROJECT LOGO -->
<div align="center">
<img src="https://raw.githubusercontent.com/bobertoyin/bobertoyin/main/mbta-rs-logo.png" alt="MBTA Logo" width="80" height="80">
<h2>MBTA-RS</h2>
<p>A simple Rust client for interacting with the <a href="https://www.mbta.com/developers/v3-api">Massachusetts Bay Transport Authority's V3 API</a>*</p>
<small>*This project is not affiliated with any official development work from the MBTA</small>
</div>

<!-- CONTRIBUTING -->
## Contributing

### Getting Started

### Branches

### Semantic Versioning

<!-- ROADMAP -->
## Roadmap

This section is here to document the planning of potential features and current maintenance. 
Generally speaking, maintenance tasks tend to be a definite necessity, whereas features may or may not come to fruition.
Any of these categories would be a good start for contribution.

### **MAINTENANCE** - Improving Unit/Integration Testing

Current unit testing implementation is pretty weak, as it's just

1. copy a ~20 data object-sized response from the real API into a JSON file
2. load JSON file into an HTTP mocker and just check for serialization at the correct endpoint

Most sub-structs of data models are missing real unit tests (especially ones with custom serialization/deserialization methods), so it'd be good to

a. document what needs thorough unit testing
b. test those things!

There also needs to be some kind of "smoke test" that interacts with the real MBTA API to keep the client up-to-date and catch any unexpected schemas/fields.

### **MAINTENANCE** - API Scouting/Documentation Improvment

There's not much to be done here other than just keep an eye on fishy doc-comments in this library and confirming that the original docs are either just as fishy or need to be cleaned up a bit.

### **MAINTENANCE** - Establishing Standardized Contribution/Git/Github Guidelines and Best Practices

Here are some following things to document for contributors:

- necessary installs/dependencies
- code formatting/linting
- where to look for starter tasks
- naming conventions (important for macros!)
- testing conventions
- [convential commits](https://www.conventionalcommits.org/en/v1.0.0/)
- CI/CD
- PR conventions

Here are some necessary things to setup:
- issue template
- PR template
- branch protection?

### **MAINTENANCE** - All the CI/CD Things

Configure a Github Actions pipeline with the following necessary elements:

- [x] `cargo test` step
- [x] `cargo clippy` step
- [x] `cargo fmt check` step

and potential "nice to have" steps:

- [convential commits](https://www.conventionalcommits.org/en/v1.0.0/) check

### **MAINTENANCE** - Benchmarks and Examples

It'd be nice to have some simple benchmarks and examples for this crate.

Potential benchmark candidates:
    - https://github.com/mvertescher/mbta-v3-swagger-api-client-rs
    - https://github.com/johnsliao/pymbta
    - https://github.com/milas/python-mbta

### **FEATURE** - Optional Visualization Module

### **FEATURE** - Support for Event Streaming

[Event Streaming for the API](https://www.mbta.com/developers/v3-api/streaming) is significantly more useful for real-time applications, so support for this feature would be nice. Planning for this feature needs to be fleshed out, but the general steps are:

- implement custom parsing for the streamed data, as it isn't quite JSON per message
- decide whether to bake functionality into existing client or move into its own "streamingclient"
- determine whether or not this functionality requires async features to work
