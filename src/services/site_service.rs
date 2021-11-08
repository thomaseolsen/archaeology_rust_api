use crate::models::site_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 * POST method for the Site.
 * Writes the passed object to the database.
 */
pub async fn post_site(data: web::Json<site_model::Site>) -> Result<web::Json<site_model::Site>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

/*
 * GET method for the Site.
 * Queries the requested object from the database.
 */
pub async fn get_site(req: HttpRequest) -> impl Responder {
    let site = req.match_info().get("site").unwrap_or("Unknown");
    format!("Welcome to Site: {}", &site)
}
