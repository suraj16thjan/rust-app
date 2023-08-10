use actix_web::{middleware::Logger, App, HttpServer};
mod config;
mod modules;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::router::api)
    })
    .bind(config.bind_address)?
    .run()
    .await
}
