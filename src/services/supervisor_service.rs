use crate::models::supervisor_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 *
 */
pub async fn post_supervisor(data: web::Json<supervisor_model::Supervisor>) -> Result<web::Json<supervisor_model::Supervisor>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

pub async fn get_supervisor(req: HttpRequest) -> impl Responder {
    let supervisor = req.match_info().get("supervisor").unwrap_or("Unknown");
    format!("Welcome to Supervisor: {}", &supervisor)
}
