use async_graphql::*;

use super::{
    types::{Coordinate, RawCoordinate},
    utils,
};

#[derive(Default)]
pub struct MappingMutations;

#[Object]
impl MappingMutations {
    async fn parse_coordinates(
        &self,
        coordinates: Vec<RawCoordinate>,
    ) -> Result<Vec<Coordinate>, Error> {
        let parsed = utils::parse_coordinates(coordinates)?;
        Ok(parsed)
    }
}
