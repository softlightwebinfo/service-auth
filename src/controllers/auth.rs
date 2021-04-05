use actix_web::{delete, get, HttpRequest, HttpResponse, post, put, Result};
use actix_web::Responder;
use actix_web::web::{Data, Json};

use crate::constants;
use crate::db::Pool;
use crate::models::auth::{RQLogin, User, UserDTO};
use crate::responses::response::ResponseBody;
use crate::services::account_service;

#[post("login")]
pub async fn auth(
    body: Json<RQLogin>,
    pool: Data<Pool>,
) -> Result<HttpResponse> {
    match account_service::login(body.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}

#[post("/logout")]
pub async fn logout(req: HttpRequest, pool: Data<Pool>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        account_service::logout(authen_header, &pool);
        Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}

#[get("/user")]
pub async fn get_user_by_id(req: HttpRequest, pool: Data<Pool>) -> Result<HttpResponse> {
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        match account_service::get_user_by_id(authen_header, &pool) {
            Ok(user) => { Ok(HttpResponse::Ok().json(ResponseBody::<User>::new(constants::MESSAGE_GET_USER_SUCCESS, user))) }
            Err(_) => { Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY))) }
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}

#[get("")]
pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}


#[post("/signup")]
pub async fn signup(user_dto: Json<UserDTO>, pool: Data<Pool>) -> Result<HttpResponse> {
    match account_service::signup(user_dto.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_SIGNUP_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}


#[delete("/")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}

#[put("/")]
pub async fn put_user() -> impl Responder {
    format!("hello from delete user")
}