use actix_web::{middleware::Logger, App, HttpServer};
mod config;
mod modules;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::router::api)
    })
    .bind(config.bind_address)?
    .run()
    .await
}
