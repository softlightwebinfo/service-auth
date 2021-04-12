use actix_web::{get, HttpResponse, Result};
use actix_web::web::Data;

use crate::constants;
use crate::db::Pool;
use crate::responses::response::ResponseBody;
use crate::services::web_service;

#[get("")]
pub async fn webs(pool: Data<Pool>) -> Result<HttpResponse> {
    match web_service::get_webs(&pool) {
        Ok(web) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, web))),
        Err(err) => Ok(err.response()),
    }
}
