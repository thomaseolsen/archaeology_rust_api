use crate::models::area_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 *
 */
pub async fn post_area(data: web::Json<area_model::Site>) -> Result<web::Json<area_model::Site>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

pub async fn get_area(req: HttpRequest) -> impl Responder {
    let area = req.match_info().get("area").unwrap_or("Unknown");
    format!("Welcome to Area: {}", &area)
}
