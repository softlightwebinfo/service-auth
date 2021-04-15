use actix_web::http::StatusCode;
use actix_web::web::Data;

use crate::constants;
use crate::db::Pool;
use crate::models::web::Web;
use crate::services::service_error::ServiceError;

pub fn get_webs(pool: &Data<Pool>) -> Result<Vec<Web>, ServiceError> {
    if let Ok(webs) = Web::find_all(&pool.get().unwrap()) {
        return Ok(webs);
    }
    Err(ServiceError::new(StatusCode::OK, constants::MESSAGE_ERROR_NOT_FOUND.to_string()))
}

pub fn create_web(pool: &Data<Pool>, body: Web) -> bool {
    Web::create(&pool.get().unwrap(), body)
}