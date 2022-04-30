use crate::models::area_model;
use crate::models::associations::asc_area_team_model;
use actix_web::{http, HttpRequest, Responder, web};

/*
 * POST method for the Area.
 * Writes the passed object to the database.
 */
pub async fn post_area(data: web::Json<area_model::Area>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
        .customize()
        .with_status(http::StatusCode::CREATED)
}

/*
 * GET method for the Area.
 * Queries the requested object from the database.
 */
pub async fn get_area(req: HttpRequest) -> impl Responder {
    let area = req.match_info().get("area").unwrap_or("Unknown");
    format!("Welcome to Area: {}", &area)
}

/*
 * POST method to assign a Team to an Area.
 * Writes the passed object to the database.
 */
pub async fn post_area_team(data: web::Json<asc_area_team_model::AscAreaTeam>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
    .customize()
        .with_status(http::StatusCode::CREATED)
}
