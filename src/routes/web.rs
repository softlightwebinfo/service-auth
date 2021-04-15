use actix_web::web;

use crate::controllers;

pub fn web_configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("webs")
            .service(controllers::web::webs)
            .service(controllers::web::post_webs)
    );
}
