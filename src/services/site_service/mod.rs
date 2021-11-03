use actix_web::{web, HttpResponse};

pub fn site_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| HttpResponse::Ok().body("This is a Site!")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
