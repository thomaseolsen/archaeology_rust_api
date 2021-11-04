mod configs;
mod models;
mod services;

use actix_web::{web, App, HttpServer, Responder};

async fn welcome() -> impl Responder {
    format!("Welcome to the Archaeology API written in Rust!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(welcome))
            .service(web::scope("/services/area").configure(configs::area_config))
            .service(web::scope("/services/site").configure(configs::site_config))
            .service(web::scope("/services/team").configure(configs::team_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
