use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/attendance")
            .route("/clockin", web::post().to(handlers::clock_in))
            .route("/clockout", web::post().to(handlers::clock_out))
            .route("", web::get().to(handlers::get_history)),
    )
    .service(
        web::scope("/api/banners")
            .route("/upload", web::post().to(handlers::upload_banner_image))
            .route("", web::post().to(handlers::create_banner))
            .route("", web::get().to(handlers::get_banners))
            .route("/active", web::get().to(handlers::get_active_banner))
            .route("/{id}/image", web::put().to(handlers::update_banner_image))
            .route("/{id}", web::put().to(handlers::update_banner))
            .route("/{id}", web::delete().to(handlers::delete_banner)),
    );
}
