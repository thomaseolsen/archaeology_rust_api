use crate::models::supervisor_model;
use actix_web::{http, HttpRequest, Responder, web};

/*
 * POST method for the Supervisor.
 * Writes the passed object to the database.
 */
pub async fn post_supervisor(data: web::Json<supervisor_model::Supervisor>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
        .customize()
        .with_status(http::StatusCode::CREATED)
}

/*
 * GET method for the Supervisor.
 * Queries the requested object from the database.
 */
pub async fn get_supervisor(req: HttpRequest) -> impl Responder {
    let supervisor = req.match_info().get("supervisor").unwrap_or("Unknown");
    format!("Welcome to Supervisor: {}", &supervisor)
}
