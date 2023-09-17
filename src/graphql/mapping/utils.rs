use async_graphql::{Error, Result};

use super::types::{Coordinate, RawCoordinate};

pub fn parse_coordinates(coordinates: Vec<RawCoordinate>) -> Result<Vec<Coordinate>, Error> {
    let mut parsed: Vec<Coordinate> = vec![];
    let mut id = 0;
    for coord in coordinates {
        let lon_vals = parse_str_coordinate(coord.lon, CoordType::Longitude)?;
        let lat_vals = parse_str_coordinate(coord.lat, CoordType::Latitude)?;
        parsed.push(Coordinate::new(
            id, lat_vals.0, lon_vals.0, lat_vals.1, lon_vals.1,
        ));
        id += 1;
    }

    Ok(parsed)
}

enum CoordType {
    Latitude,
    Longitude,
}

fn parse_str_coordinate(coord: String, coord_type: CoordType) -> Result<(f32, String), Error> {
    let lon_parts: Vec<&str> = coord.split(",").collect();
    let deg: &str = lon_parts.get(0).unwrap_or(&"0");
    let min: &str = lon_parts.get(1).unwrap_or(&"0");
    let sec: &str = lon_parts.get(2).unwrap_or(&"0");

    let direction = match coord_type {
        CoordType::Latitude => "N",
        CoordType::Longitude => "E",
    };

    let lon_str = format!("{deg}° {min}' {sec}{} {}", "\"", direction);

    let deg_f32: f32 = deg.parse::<f32>()?;
    let min_f32: f32 = min.parse::<f32>()?;
    let sec_f32: f32 = min.parse::<f32>()?;

    let lon: f32 = deg_f32 + (min_f32 / 60.0) + (sec_f32 / 3600.0);
    return Ok((lon, lon_str));
}
