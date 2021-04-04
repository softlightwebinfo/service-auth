use crate::db::Connection;
use crate::models::auth::{RQLogin, User};

impl User {
    pub fn login(rq: RQLogin, conn: &Connection) {
        println!("{:?}", rq)
    }
    pub fn signup() {}
    pub fn logout() {}
    pub fn is_valid_login_session() {}
    pub fn find_user_by_username() {}
    pub fn generate_login_session() {}
    pub fn update_login_session_to_db() {}
}
