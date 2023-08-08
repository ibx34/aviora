use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Civ {
    pub id: String,
    pub r#ref: String,
    pub unique_state_id: String,
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
    pub date_of_birth: NaiveDate,
    pub race: String,
    pub whitelists: Option<Vec<String>>,
    pub ip_that_created: std::net::IpAddr,
    pub created_at: NaiveDateTime,
}

impl Civ {
    pub fn short_ref(&self) -> &str {
        &self.r#ref[0..4]
    }
}
