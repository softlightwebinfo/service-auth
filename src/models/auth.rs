use serde::{Deserialize, Serialize};

use super::super::schema::*;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub login_session: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RQLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct LoginInfoDTO {
    pub email: String,
    pub name: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
}
