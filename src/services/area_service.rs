use crate::models::area_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 * POST method for the Area.
 * Writes the passed object to the database.
 */
pub async fn post_area(data: web::Json<area_model::Area>) -> Result<web::Json<area_model::Area>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

/*
 * GET method for the Area.
 * Queries the requested object from the database.
 */
pub async fn get_area(req: HttpRequest) -> impl Responder {
    let area = req.match_info().get("area").unwrap_or("Unknown");
    format!("Welcome to Area: {}", &area)
}
