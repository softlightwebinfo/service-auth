use serde::{Deserialize, Serialize};

use super::super::schema::*;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RQLogin {
    pub username: String,
    pub password: String,
}
