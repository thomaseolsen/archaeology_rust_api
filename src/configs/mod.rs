use crate::services;
use actix_web::{web, HttpResponse};

pub fn area_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::area_service::post_area));
    cfg.route("/{area}", web::get().to(services::area_service::get_area));
    cfg.route("/assign_team/", web::post().to(services::area_service::post_area_team));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}

pub fn locus_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::locus_service::post_locus));
    cfg.route("/{locus}", web::get().to(services::locus_service::get_locus));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}

pub fn site_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::site_service::post_site));
    cfg.route("/{site}", web::get().to(services::site_service::get_site));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}

pub fn square_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::square_service::post_square));
    cfg.route("/{square}", web::get().to(services::square_service::get_square));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}

pub fn supervisor_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::supervisor_service::post_supervisor));
    cfg.route("/{supervisor}", web::get().to(services::supervisor_service::get_supervisor));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}

pub fn team_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::post().to(services::team_service::post_team));
    cfg.route("/{team}", web::get().to(services::team_service::get_team));
    cfg.route("/", web::head().to(|| HttpResponse::MethodNotAllowed()));
}
