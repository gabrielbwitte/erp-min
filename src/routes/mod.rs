use actix_web::web;

use crate::handlers::auth::login;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(login));
}
