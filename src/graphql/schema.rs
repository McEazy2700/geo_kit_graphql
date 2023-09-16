use async_graphql::*;

use super::mapping::mutations::MappingMutations;

pub struct Query;

#[Object]
impl Query {
    async fn version(&self) -> String {
        String::from("0.0.1")
    }
}

#[derive(MergedObject, Default)]
pub struct Mutation(MappingMutations);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    AppSchema::new(Query, Mutation::default(), EmptySubscription)
}
