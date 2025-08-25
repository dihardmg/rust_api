use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/attendance")
            .route("/clockin", web::post().to(handlers::clock_in))
            .route("/clockout", web::post().to(handlers::clock_out))
            .route("", web::get().to(handlers::get_history)),
    );
}
