use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RQPostWeb {
    pub web: String,
    pub url: String,
    pub token: String,
    pub image: String,
}
