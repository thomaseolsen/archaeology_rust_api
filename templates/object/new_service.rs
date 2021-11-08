use crate::models::newobject_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 * POST method for the newobject.
 * Writes the passed object to the database.
 */
pub async fn post_newobject(data: web::Json<newobject_model::NewObject>) -> Result<web::Json<newobject_model::NewObject>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

/*
 * GET method for the newobject.
 * Queries the requested object from the database.
 */
pub async fn get_newobject(req: HttpRequest) -> impl Responder {
    let newobject = req.match_info().get("newobject").unwrap_or("Unknown");
    format!("Welcome to NewObject: {}", &newobject)
}
