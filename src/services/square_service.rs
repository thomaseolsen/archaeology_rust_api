use crate::models::square_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 * POST method for the Square.
 * Writes the passed object to the database.
 */
pub async fn post_square(data: web::Json<square_model::Square>) -> Result<web::Json<square_model::Square>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

/*
 * GET method for the Square.
 * Queries the requested object from the database.
 */
pub async fn get_square(req: HttpRequest) -> impl Responder {
    let square = req.match_info().get("square").unwrap_or("Unknown");
    format!("Welcome to Square: {}", &square)
}
