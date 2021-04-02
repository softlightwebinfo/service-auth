use serde::Serialize;

#[derive(Serialize, Queryable)]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}
