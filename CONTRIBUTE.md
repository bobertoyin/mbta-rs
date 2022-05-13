## Getting Started

1. Install [Rust](https://www.rust-lang.org/tools/install) and [cargo](https://github.com/rust-lang/cargo) (included with Rust)
2. Install [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy)

## Testing

Unit tests for any given module should be under a `tests` submodule; generally only custom serialization/deserialization implementations need to be unit tested, along with custom errors and the client itself.

Model correctness and API "sanity" testing (testing against the actual API) is done under the `tests` directory: `simple.rs` are sanity tests for the simpler endpoints and utilize a simple macro to cut down on boilerplate, while the other files are for more involved endpoints. *This is hardly a good way to test model correctness and any suggestions to improve testing in this area are appreciated.*

For running the sanity tests locally, you should [register for an API key](https://api-v3.mbta.com/register) and load it as an environment variable named `MBTA_TOKEN`. This is so that the tests don't run out of requests before being completed.

## Branch Policy

There are currently no strict naming conventions for branches, simply create a new branch and open a PR for it to be merged into main.

## Continuous Integration

The main CI pipeline declaration lives in the `test.yml` file under the `.github/workflows` directory.

Pipeline commands:
```shell
# clippy linting
cargo clippy --all-features -- -D warnings
# format checking
cargo fmt --all -- --check
# run tests
cargo test
```

Commands to run locally:
```shell
# clippy linting
cargo clippy --all-features -- -D warnings
# apply formatting
cargo fmt --all
# run tests
cargo test
```

See the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for how to format your git commit messages; there are no current formalities, so choose a commit message that works best for the changes you are making.

# Versioning

See [SemVer specification](https://semver.org/) for the general idea behind semantic versioning, but versioning for this project will deviant from the SemVer specification somewhat to better match Rust library conventions.

`PATCH` versioning will remain the same, being bumped whenever a bug fix is introduced. However, while this library is in `0.x.x`, `MINOR` version bumps *can* include breaking changes. This is because `0.x.x` releases are generally unstable in their APIs and signals to Rustaceans that the library still needs time to mature. However, once either `1.0.0` is released or some `0.x.x` release is deemed stable enough, breaking changes beyond that point warrant a `MAJOR` version bump, while backwards-compatible feature addition use the `MINOR` version bump instead.

### Releasing

Currently, only Robert Yin can make releases. There should also be an accompanying git tag for each release.

<!-- DESIGN AND CONVENTIONS -->
## Design and Conventions

TODO
