use crate::controllers::*;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(index::get)
        .service(index::get)
        .service(echo::get)
        .service(echo::post)
        .service(scrape::get)
        .service(animes::get);
}

pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg.service(callback::get);
}

pub fn health_check(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::get);
}
