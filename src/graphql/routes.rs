use actix_web::{get, post, web::Data, HttpResponse, Result};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use super::schema::AppSchema;

#[post("/graphql")]
pub async fn execute(schema: Data<AppSchema>, req: GraphQLRequest) -> Result<GraphQLResponse> {
    Ok(schema.execute(req.into_inner()).await.into())
}

#[get("/graphql")]
pub async fn playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}
