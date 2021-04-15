use actix_web::{get, HttpResponse, post, Result};
use actix_web::web::{Data, Json};

use crate::constants;
use crate::db::Pool;
use crate::models::web::Web;
use crate::responses::response::ResponseBody;
use crate::services::web_service;

#[get("")]
pub async fn webs(pool: Data<Pool>) -> Result<HttpResponse> {
    match web_service::get_webs(&pool) {
        Ok(web) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, web))),
        Err(err) => Ok(err.response()),
    }
}

#[post("")]
pub async fn post_webs(
    pool: Data<Pool>,
    body: Json<Web>,
) -> Result<HttpResponse> {
    match web_service::create_web(&pool, body.0) {
        true => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_SUCCESS_POST_WEBS, ""))),
        false => Ok(HttpResponse::InternalServerError().json(ResponseBody::new(constants::MESSAGE_ERROR_POST_WEBS, ""))),
    }
}
