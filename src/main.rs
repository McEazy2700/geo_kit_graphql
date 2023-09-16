use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use graphql::{
    routes::{execute, playground},
    schema::build_schema,
};
use std::env::var;

use dotenv::dotenv;

pub mod graphql;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let debug = var("DEBUG").unwrap_or(String::from("false")) == "true";
    let port = var("PORT").unwrap_or(String::from("8080"));

    let address = if debug {
        format!("127.0.0.1:{port}")
    } else {
        format!("0.0.0.0:{port}")
    };

    println!("Server started at http://{address}");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_cors::Cors::permissive())
            .app_data(Data::new(build_schema()))
            .service(hello)
            .service(playground)
            .service(execute)
    })
    .bind(address)?
    .run()
    .await
}
