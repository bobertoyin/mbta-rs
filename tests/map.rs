//! Simple tests for tile map plotting.

use std::{fs::remove_file, path::PathBuf};

use mbta_rs::{map::*, *};
use raster::{compare::similar, open};
use staticmap::*;

use rstest::*;

#[fixture]
fn client() -> Client {
    if let Ok(token) = std::env::var("MBTA_TOKEN") {
        Client::with_key(token)
    } else {
        Client::without_key()
    }
}

fn image_file(relative_path: &str) -> PathBuf {
    PathBuf::from(format!("{}/resources/test/{}", env!("CARGO_MANIFEST_DIR"), relative_path))
}

#[rstest]
fn test_simple_map_render(client: Client) {
    // Arrange
    let route_params = [("filter[type]", "0,1")];
    let routes = client.routes(&route_params).expect("failed to get routes");
    let mut map = StaticMapBuilder::new()
        .width(1000)
        .height(1000)
        .zoom(12)
        .lat_center(42.326768)
        .lon_center(-71.100099)
        .build()
        .expect("failed to build map");
    let actual_path = image_file("actual_map.png");
    let actual_path = actual_path.to_str().expect("failed to load path: `actual_map.png`");
    let expected_path = image_file("expected_map.png");
    let expected_path = expected_path.to_str().expect("failed to load path: `expected_map.png`");

    // Act
    for route in routes.data {
        let params = [("filter[route]", &route.id)];
        let shapes = client.shapes(&params).expect("failed to get shapes");
        for shape in shapes.data {
            shape
                .plot(&mut map, true, PlotStyle::new((route.attributes.color.clone(), 3.0), Some(("#FFFFFF".into(), 1.0))))
                .expect("failed to plot shape");
        }
        let stops = client.stops(&params).expect("failed to get stops");
        for stop in stops.data {
            stop.plot(&mut map, true, PlotStyle::new((route.attributes.color.clone(), 3.0), Some(("#FFFFFF".into(), 1.0))))
                .expect("failed to plot stop");
        }
        let vehicles = client.vehicles(&params).expect("failed to get vehicles");
        for vehicle in vehicles.data {
            vehicle
                .plot(&mut map, true, IconStyle::new(image_file("train.png"), 12.5, 12.5))
                .expect("failed to plot vehicle");
        }
    }
    map.save_png(image_file("actual_map.png")).expect("failed to save map to file");

    // Assert
    let actual = open(actual_path).expect("failed to open: `actual_map.png`");
    let expected = open(expected_path).expect("failed to open: `expected_map.png`");
    assert!(similar(&actual, &expected).expect("failed to compare images") <= 10);
    remove_file(actual_path).expect("failed to remove: `actual_map.png`");
}
