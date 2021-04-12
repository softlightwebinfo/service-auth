use actix_web::http::{HeaderValue, StatusCode};
use actix_web::web;
use actix_web::web::Data;

use crate::constants;
use crate::db::Pool;
use crate::models::auth::{RQLogin, User, UserDTO};
use crate::requests::rq_auth::RQPutUser;
use crate::responses::token_body::TokenBodyResponse;
use crate::services::{user_token::UserToken};
use crate::services::service_error::ServiceError;
use crate::utils::token_utils;

pub fn login(login: RQLogin, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    let connect = &pool.get().unwrap();
    match User::login(login, &connect) {
        Some(logged_user) => {
            match serde_json::from_value(json!({ "token": UserToken::generate_token(&logged_user), "token_type": "bearer", "user": logged_user })) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGIN_FAILED.to_string()))
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        }
        None => Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_USER_NOT_FOUND.to_string()))
    }
}

pub fn logout(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.to_lowercase().starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_username(&username, &pool.get().unwrap()) {
                        User::logout(user.id, &pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()))
}

pub fn get_user_by_id(authen_header: &HeaderValue, pool: &Data<Pool>) -> Result<User, String> {
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.to_lowercase().starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(username) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = User::find_user_by_username(&username, &pool.get().unwrap()) {
                        return Ok(user);
                    }
                }
            }
        }
    }
    Err(constants::MESSAGE_INVALID_TOKEN.to_string())
}

pub fn signup(user: UserDTO, pool: &Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::signup(user, &pool.get().unwrap()) {
        Ok(logged_user) => {
            match serde_json::from_value(json!({ "token": UserToken::generate_token(&logged_user), "token_type": "bearer" })) {
                Ok(token_res) => {
                    if logged_user.login_session.is_empty() {
                        Err(ServiceError::new(StatusCode::UNAUTHORIZED, constants::MESSAGE_LOGIN_FAILED.to_string()))
                    } else {
                        Ok(token_res)
                    }
                }
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        }
        Err(message) => Err(ServiceError::new(StatusCode::BAD_REQUEST, message))
    }
}

pub fn put_user(id_user: i32, user: RQPutUser, pool: &Data<Pool>) -> bool {
    User::put_user(id_user, user, &pool.get().unwrap())
}
