use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::prelude::*;
use uuid::Uuid;

use crate::constants;
use crate::db::Connection;
use crate::models::auth::{LoginInfoDTO, RQLogin, User, UserDTO};
use crate::models::login_history::LoginHistory;
use crate::schema::users::dsl::{email, login_session, users};
use crate::services::user_token::UserToken;

impl User {
    pub fn login(rq: RQLogin, conn: &Connection) -> Option<LoginInfoDTO> {
        if let Ok(user_to_verify) = users
            .filter(email.eq(&rq.username))
            .get_result::<User>(conn) {
            if !user_to_verify.password.is_empty() &&
                verify(&rq.password, &user_to_verify.password).unwrap() {
                if let Some(login_history) = LoginHistory::create(&user_to_verify.email, &conn) {
                    if LoginHistory::save_login_history(login_history, &conn).is_err() {
                        return None;
                    }
                    let login_session_str = User::generate_login_session();
                    if User::update_login_session_to_db(
                        &user_to_verify.email,
                        &login_session_str,
                        &conn,
                    ) {
                        return Some(LoginInfoDTO {
                            email: user_to_verify.email,
                            login_session: login_session_str,
                        });
                    }
                }
            }
        }
        None
    }
    pub fn signup(user: UserDTO, conn: &Connection) -> Result<LoginInfoDTO, String> {
        if Self::find_user_by_username(&user.email, conn).is_err() {
            let hashed_pwd = hash(&user.password, DEFAULT_COST).unwrap();
            let user = UserDTO {
                password: hashed_pwd,
                ..user
            };
            diesel::insert_into(users)
                .values(&user)
                .execute(conn);

            if let Some(login_history) = LoginHistory::create(&user.email, &conn) {
                if LoginHistory::save_login_history(login_history, &conn).is_err() {
                    return Err(format!("User '{}' error", &user.email));
                }
                let login_session_str = User::generate_login_session();
                if User::update_login_session_to_db(
                    &user.email,
                    &login_session_str,
                    &conn,
                ) {
                    return Ok(LoginInfoDTO {
                        email: user.email.to_string(),
                        login_session: login_session_str.to_string(),
                    })
                }
            }
            return Err(constants::MESSAGE_LOGIN_FAILED.to_string());
        }
        Err(format!("User '{}' is already registered", &user.email))
    }
    pub fn logout(user_id: i32, conn: &Connection) {
        if let Ok(user) = users.find(user_id).get_result::<User>(conn) {
            Self::update_login_session_to_db(&user.email, "", conn);
        }
    }
    pub fn is_valid_login_session(user_token: &UserToken, conn: &Connection) -> bool {
        users
            .filter(email.eq(&user_token.user))
            .filter(login_session.eq(&user_token.login_session))
            .get_result::<User>(conn)
            .is_ok()
    }
    pub fn find_user_by_username(email_rq: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(email.eq(email_rq)).get_result::<User>(conn)
    }
    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
    pub fn update_login_session_to_db(
        email_rq: &str,
        login_session_str: &str,
        conn: &Connection,
    ) -> bool {
        if let Ok(user) = User::find_user_by_username(email_rq, conn) {
            diesel::update(users.find(user.id))
                .set(login_session.eq(login_session_str.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }
}
