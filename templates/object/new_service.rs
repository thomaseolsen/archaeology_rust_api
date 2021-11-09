use crate::models::newobject_model;
use actix_web::{http, HttpRequest, Responder, web};

/*
 * POST method for the NewObject.
 * Writes the passed object to the database.
 */
pub async fn post_newobject(data: web::Json<newobject_model::NewObject>) -> impl Responder {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    data
        .with_status(http::StatusCode::CREATED)
}

/*
 * GET method for the NewObject.
 * Queries the requested object from the database.
 */
pub async fn get_newobject(req: HttpRequest) -> impl Responder {
    let newobject = req.match_info().get("newobject").unwrap_or("Unknown");
    format!("Welcome to NewObject: {}", &newobject)
}
