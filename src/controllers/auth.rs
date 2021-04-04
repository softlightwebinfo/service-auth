use actix_web::{delete, get, post, put};
use actix_web::Responder;

#[post("")]
pub async fn auth() -> String {
    format!("Hello {:?}!", "HOLA")
}

#[get("")]
pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}

#[get("")]
pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

#[post("/")]
pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

#[delete("/")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}

#[put("/")]
pub async fn put_user() -> impl Responder {
    format!("hello from delete user")
}