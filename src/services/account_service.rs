use actix_web::web;

use crate::db::Pool;
use crate::models::auth::{User, RQLogin};

pub fn login(login: RQLogin, pool: &web::Data<Pool>) {
    match User::login(login, &pool.get().unwrap()) {
        _ => {}
    }
}
