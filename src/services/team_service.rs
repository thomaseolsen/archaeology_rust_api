use crate::models::team_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 *
 */
pub async fn post_team(data: web::Json<team_model::Team>) -> Result<web::Json<team_model::Team>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

pub async fn get_team(req: HttpRequest) -> impl Responder {
    let team = req.match_info().get("team").unwrap_or("Unknown");
    format!("Welcome to Team: {}", &team)
}
