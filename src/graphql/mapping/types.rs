use async_graphql::*;

#[derive(SimpleObject, InputObject)]
pub struct RawCoordinate {
    pub lat: String,
    pub lon: String,
}

#[derive(SimpleObject)]
pub struct Coordinate {
    pub lat_str: String,
    pub lon_str: String,
    pub lat: f32,
    pub lon: f32,
}

impl Coordinate {
    pub fn new(lat: f32, lon: f32, lat_str: String, lon_str: String) -> Self {
        Self {
            lat_str,
            lon_str,
            lon,
            lat,
        }
    }
}
