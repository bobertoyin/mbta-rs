//! Module for plotting models that contain location data onto a tile map.

use std::path::PathBuf;

use colors_transform::{Color, ParseError, Rgb};
use polyline::decode_polyline;
use staticmap::{
    tools::{CircleBuilder, Color as MapColor, IconBuilder, LineBuilder},
    Error as MapError, StaticMap,
};
use thiserror::Error;

use super::*;

/// Errors that can occur when plotting.
#[derive(Error, Debug)]
pub enum PlotError {
    /// Color conversion parsing failed.
    #[error("color conversion failed during parsing: `{0}`")]
    ColorError(String),
    /// Map related errors.
    #[error("map error: `{0}`")]
    MapError(#[from] MapError),
    /// Polyline error.
    #[error("polyline error: `{0}`")]
    PolylineError(String),
}

impl From<ParseError> for PlotError {
    fn from(error: ParseError) -> Self {
        PlotError::ColorError(error.message)
    }
}

/// Convert RGB color representation from the [colors_transform] crate into color representation from the [staticmap] crate.
/// Automatically sets alpha value to 100%, a.k.a this conversion does not support transparency values.
///
/// # Arguments
///
/// * `rgb` - RGB representation from [colors_transform]
/// * `anti_alias` - whether to use anti-aliasing
///
/// ```
/// use colors_transform::Rgb;
/// use staticmap::tools::Color;
/// use mbta_rs::map::*;
///
/// // black RGB color
/// let rgb = Rgb::from(0.0, 0.0, 0.0);
/// // convert with anti-aliasing
/// let converted = rgb_to_map_color(rgb, true);
/// ```
pub fn rgb_to_map_color(rgb: Rgb, anti_alias: bool) -> MapColor {
    MapColor::new(anti_alias, rgb.get_red() as u8, rgb.get_green() as u8, rgb.get_blue() as u8, 255)
}

/// Plotting styles for models; typically hex colors and widths/radii.
#[derive(Debug, Clone, PartialEq)]
pub struct PlotStyle {
    /// Inner hex color string and width/radius.
    pub inner: (String, f32),
    /// Optional border hex color string and width.
    pub border: Option<(String, f32)>,
}

impl PlotStyle {
    /// Create a new [PlotStyle].
    ///
    /// # Arguments
    ///
    /// * `inner` - inner data
    /// * `border` - border data
    ///
    /// ```
    /// use mbta_rs::map::*;
    ///
    /// // create a style with an inner color of white and 3.0 pixel width/radius
    /// // and a black border of 1.0 pixel width
    /// let style = PlotStyle::new(("#FFFFFF".into(), 3.0), Some(("#000000".into(), 1.0)));
    /// ```
    pub fn new(inner: (String, f32), border: Option<(String, f32)>) -> Self {
        Self { inner, border }
    }
}

/// Plotting style for model icons.
#[derive(Debug, Clone, PartialEq)]
pub struct IconStyle {
    /// Path to icon file.
    pub icon: PathBuf,
    /// X-offset in pixels from the bottom-left of the map.
    pub x_offset: f64,
    /// Y-offset in pixels from the bottom-left of the map.
    pub y_offset: f64,
}

impl IconStyle {
    /// Create a new [IconStyle].
    ///
    /// # Arguments
    ///
    /// * `icon` - path to icon file
    /// * `x_offset` - x-offset in pixels from bottom-left of the map
    /// * `y_offset` - y-offset in pixels from bottom-left of the map
    ///
    /// ```
    /// use mbta_rs::map::*;
    ///
    /// // create a style from "foobar.png" with no offsets
    /// let style = IconStyle::new("foobar.png", 0.0, 0.0);
    /// ```
    pub fn new<P: Into<PathBuf>>(icon: P, x_offset: f64, y_offset: f64) -> Self {
        Self {
            icon: icon.into(),
            x_offset,
            y_offset,
        }
    }
}

/// Trait for data models that can plotted onto a tile map.
pub trait Plottable<D> {
    /// Plot this model onto a tile map.
    ///
    /// # Arguments
    ///
    /// * `map` - mutable reference to a tile map
    /// * `anti_alias` - whether to render with anti-aliasing or not
    /// * `plot_style` - plot style for the model
    fn plot(self, map: &mut StaticMap, anti_alias: bool, extra_data: D) -> Result<(), PlotError>;
}

impl Plottable<PlotStyle> for Stop {
    fn plot(self, map: &mut StaticMap, anti_alias: bool, plot_style: PlotStyle) -> Result<(), PlotError> {
        if let Some(border_data) = plot_style.border {
            let border = CircleBuilder::new()
                .lat_coordinate(self.attributes.latitude)
                .lon_coordinate(self.attributes.longitude)
                .color(rgb_to_map_color(Rgb::from_hex_str(border_data.0.as_str())?, anti_alias))
                .radius(border_data.1 + plot_style.inner.1)
                .build()?;
            map.add_tool(border);
        }
        let inner_circle = CircleBuilder::new()
            .lat_coordinate(self.attributes.latitude)
            .lon_coordinate(self.attributes.longitude)
            .color(rgb_to_map_color(Rgb::from_hex_str(plot_style.inner.0.as_str())?, anti_alias))
            .radius(plot_style.inner.1)
            .build()?;
        map.add_tool(inner_circle);
        Ok(())
    }
}

impl Plottable<IconStyle> for Vehicle {
    fn plot(self, map: &mut StaticMap, _anti_alias: bool, icon_style: IconStyle) -> Result<(), PlotError> {
        let icon = IconBuilder::new()
            .lat_coordinate(self.attributes.latitude)
            .lon_coordinate(self.attributes.longitude)
            .x_offset(icon_style.x_offset)
            .y_offset(icon_style.y_offset)
            .path(icon_style.icon)?
            .build()?;
        map.add_tool(icon);
        Ok(())
    }
}

impl Plottable<PlotStyle> for Shape {
    fn plot(self, map: &mut StaticMap, anti_alias: bool, plot_style: PlotStyle) -> Result<(), PlotError> {
        let points = decode_polyline(&self.attributes.polyline, 5).map_err(PlotError::PolylineError)?;
        if let Some(border_data) = plot_style.border {
            let border = LineBuilder::new()
                .lat_coordinates(points.0.iter().map(|p| p.y))
                .lon_coordinates(points.0.iter().map(|p| p.x))
                .color(rgb_to_map_color(Rgb::from_hex_str(border_data.0.as_str())?, anti_alias))
                .width(border_data.1 + plot_style.inner.1)
                .build()?;
            map.add_tool(border);
        }
        let inner_line = LineBuilder::new()
            .lat_coordinates(points.0.iter().map(|p| p.y))
            .lon_coordinates(points.0.iter().map(|p| p.x))
            .color(rgb_to_map_color(Rgb::from_hex_str(plot_style.inner.0.as_str())?, anti_alias))
            .width(plot_style.inner.1)
            .build()?;
        map.add_tool(inner_line);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[fixture]
    fn color_error() -> ParseError {
        ParseError { message: "foobar".into() }
    }

    #[fixture]
    fn map_error() -> MapError {
        MapError::InvalidSize
    }

    #[rstest]
    fn test_plot_error_from_color_error(color_error: ParseError) {
        // Arrange

        // Act
        let actual = PlotError::from(color_error.clone());

        // Assert
        if let PlotError::ColorError(s) = actual {
            assert_eq!(s, color_error.message);
        } else {
            panic!("incorrect plot error")
        }
    }

    #[rstest]
    fn test_plot_error_from_map_error(map_error: MapError) {
        // Arrange

        // Act
        let actual = PlotError::from(map_error);

        // Assert
        if let PlotError::MapError(e) = actual {
            assert_eq!(format!("{:?}", e), format!("{:?}", MapError::InvalidSize));
        } else {
            panic!("incorrect plot error")
        }
    }

    #[rstest]
    fn test_plot_error_display(color_error: ParseError, map_error: MapError) {
        // Arrange
        let color_expected = format!("color conversion failed during parsing: `{}`", &color_error.message);
        let map_expected = format!("map error: `{map_error}`");
        let polyline_expected = "polyline error: `foobar`";

        // Act
        let color_actual = format!("{}", PlotError::from(color_error));
        let map_actual = format!("{}", PlotError::from(map_error));
        let polyline_actual = format!("{}", PlotError::PolylineError("foobar".into()));

        // Assert
        assert_eq!(color_actual, color_expected);
        assert_eq!(map_actual, map_expected);
        assert_eq!(polyline_actual, polyline_expected);
    }
}
