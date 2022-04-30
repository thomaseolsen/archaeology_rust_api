use crate::models::locus_model;
use actix_web::{http, HttpRequest, Responder, web};

/*
 * POST method for the Locus.
 * Writes the passed object to the database.
 */
pub async fn post_locus(data: web::Json<locus_model::Locus>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
        .customize()
        .with_status(http::StatusCode::CREATED)
}

/*
 * GET method for the Locus.
 * Queries the requested object from the database.
 */
pub async fn get_locus(req: HttpRequest) -> impl Responder {
    let locus = req.match_info().get("locus").unwrap_or("Unknown");
    format!("Welcome to Locus: {}", &locus)
}
