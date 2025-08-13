// use actix_web::{web::{self, Json}, App, HttpServer};
// use serde::{Serialize};


// #[tokio::main]
// async fn main() {
//     HttpServer::new(|| { 
//         App::new()
//         .route("/", web::get().to(welcome))
//         .route("/user_info", web::get().to(user_info))
//         }
//         )
//         .bind(("localhost", 8080))
//         .unwrap()
//         .run()
//         .await
//         .unwrap();
// }

// async fn welcome() -> String {
//     "Hello from RUST server!".to_string()
// }


// #[derive(Serialize)]
// struct User {
//     name : String ,
//     age : i32,
//     email : String,
//     profession : String
// }

// async fn user_info() -> Json<User> {
//     let user = User {
//         name : "VinitNagar".to_string(),
//         age : 21,
//         email : "vinitnagar56@gmail.com".to_string(),
//         profession : "Just Engineer".to_string()
//     };
//     Json(user)
// }

mod routes;
mod controllers;
mod services;
mod models;

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::init)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}