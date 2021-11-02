use actix_web::{web, HttpResponse};

pub fn test_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("This is a test!")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
