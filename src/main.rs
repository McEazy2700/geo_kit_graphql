use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use graphql::{
    routes::{execute, playground},
    schema::build_schema,
};

pub mod graphql;

#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(build_schema()))
            .service(hello)
            .service(playground)
            .service(execute)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
