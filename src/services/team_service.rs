use crate::models::team_model;
use actix_web::{http, HttpRequest, Responder, web};

/*
 * POST method for the Team.
 * Writes the passed object to the database.
 */
pub async fn post_team(data: web::Json<team_model::Team>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
        .customize()
        .with_status(http::StatusCode::CREATED)
}

/*
 * GET method for the Team.
 * Queries the requested object from the database.
 */
pub async fn get_team(req: HttpRequest) -> impl Responder {
    let team = req.match_info().get("team").unwrap_or("Unknown");
    format!("Welcome to Team: {}", &team)
}
