use actix_web::{delete, get, post, put};
use actix_web::Responder;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RQAuth {
    pub email: String,
    pub password: String,
}


#[post("")]
pub async fn auth(
    info: Json<RQAuth>
) -> String {
    format!("Hello {:?}!", info.email)
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