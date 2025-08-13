use actix_web::{HttpResponse};
use crate::services::user_service;
use actix_web::web::Json;

pub async fn welcome() -> HttpResponse {
    HttpResponse::Ok().body("Hello vinit!")
}
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("I am good")
}

pub async fn user_info() -> Json<crate::models::user::User> {
    let user = user_service::get_user_info();
    Json(user)
}


// actix use serce behind the scenes
// pub async fn user_info2() -> HttpResponse {
//     let user = user_service::get_user_info();
//     HttpResponse::Ok()
//         .json(user)  
// }