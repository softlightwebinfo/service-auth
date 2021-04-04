use serde::{Deserialize, Serialize};

use super::super::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}
