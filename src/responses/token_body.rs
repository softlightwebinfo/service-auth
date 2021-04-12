use crate::models::auth::LoginInfoDTO;

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
    pub user: LoginInfoDTO,
}
