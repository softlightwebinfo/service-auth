use actix_web::web;
use crate::controllers;

pub fn auth_configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("auth")
            .service(controllers::auth::auth)
            .service(controllers::auth::get_user_by_id)
            .service(controllers::auth::get_users)
            .service(controllers::auth::add_user)
            .service(controllers::auth::delete_user)
            .service(controllers::auth::put_user)
    );
}