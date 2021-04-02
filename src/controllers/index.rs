use actix_web::{get, web};

use crate::state::app_state::AppState;

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}!", app_name)
}