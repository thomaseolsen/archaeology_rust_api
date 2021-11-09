use crate::models::site_model;
use actix_web::{http, HttpRequest, Responder, web};

/*
 * POST method for the Site.
 * Writes the passed object to the database.
 */
pub async fn post_site(data: web::Json<site_model::Site>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
        .with_status(http::StatusCode::CREATED)
}

/*
 * GET method for the Site.
 * Queries the requested object from the database.
 */
pub async fn get_site(req: HttpRequest) -> impl Responder {
    let site = req.match_info().get("site").unwrap_or("Unknown");
    format!("Welcome to Site: {}", &site)
}
