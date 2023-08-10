use crate::modules;
use actix_web::web;
use actix_web::web::get;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/health").route("", get().to(modules::health::controllers::index)),
        ),
    );
}
