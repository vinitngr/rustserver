use actix_web::web;
use crate::controllers::user_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(user_controller::welcome))
       .route("/user_info", web::get().to(user_controller::user_info))
       .route("/user_info2",  web::get().to(user_controller::user_info))
       .route("/health",  web::get().to(user_controller::health_check));
}
