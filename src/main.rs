mod config;
mod handlers;
mod models;
mod responses;
mod routes;
mod entity;

use actix_web::{App, HttpServer, middleware::DefaultHeaders};
use actix_web::http::header;
use dotenvy::dotenv;
use std::env;
use env_logger;
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Create uploads directory if it doesn't exist
    fs::create_dir_all("./uploads/banners").unwrap_or_else(|e| {
        eprintln!("Warning: Failed to create uploads directory: {}", e);
    });

    let db = config::init_db().await;

    let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind = format!("{}:{}", host, port);

    println!("Server running at http://{}", bind);

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(db.clone()))
            .wrap(DefaultHeaders::new().add((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")))
            .service(actix_files::Files::new("/uploads", "./uploads").show_files_listing())
            .configure(routes::configure)
    })
        .bind(bind)?
        .run()
        .await
}
