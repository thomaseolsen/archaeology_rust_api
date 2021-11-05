use crate::models::newobject_model;
use actix_web::{HttpRequest, Responder, Result, web};

/*
 *
 */
pub async fn post_newobject(data: web::Json<newobject_model::NewObject>) -> Result<web::Json<newobject_model::NewObject>> {
    // Not doing anything, just returning a confirmation that we've received the data packet.
    Ok(data)
}

pub async fn get_newobject(req: HttpRequest) -> impl Responder {
    let newobject = req.match_info().get("newobject").unwrap_or("Unknown");
    format!("Welcome to NewObject: {}", &newobject)
}
