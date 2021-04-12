use serde::{Deserialize, Serialize};

use super::super::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "webs"]
pub struct Web {
    pub web: String,
    pub token: String,
    pub url: String,
    pub image: Option<String>,
}
