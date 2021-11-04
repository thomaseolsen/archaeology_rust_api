use crate::services;
use actix_web::{web, HttpResponse};

pub fn area_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::area_service::post_area));
    cfg.route("/{area}", web::get().to(services::area_service::get_area));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}

pub fn site_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::site_service::post_site));
    cfg.route("/{site}", web::get().to(services::site_service::get_site));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}
