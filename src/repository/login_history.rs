use crate::models::login_history::{LoginHistory, LoginHistoryInsertableDTO};
use crate::db::Connection;
use crate::models::auth::User;
use chrono::Utc;
use crate::schema::login_history::dsl::login_history;
use diesel::{QueryResult, RunQueryDsl};

impl LoginHistory {
    pub fn create(un: &str, conn: &Connection) -> Option<LoginHistoryInsertableDTO> {
        if let Ok(user) = User::find_user_by_username(un, conn) {
            let now = Utc::now();
            Some(LoginHistoryInsertableDTO {
                user_id: user.id,
                login_timestamp: now.naive_utc(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: LoginHistoryInsertableDTO, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(login_history)
            .values(&insert_record)
            .execute(conn)
    }
}