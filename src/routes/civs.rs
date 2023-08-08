use std::net::SocketAddr;

use crate::{app::App, config::ALLOWED_RACES, models::Civ};
use axum::{
    extract::{
        ConnectInfo, Json, State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct RegisterNewCivReq {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
    pub date_of_birth: NaiveDate,
    pub race: String,
}

/// POST /civs/register
/// Welcome to the city!
pub async fn register_new_civ(
    State(app): State<App>,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    Json(civ): Json<RegisterNewCivReq>,
) -> impl IntoResponse {
    if !ALLOWED_RACES.contains(&civ.race.as_ref()) {
        return (StatusCode::BAD_REQUEST, json!({"code":"0"}).to_string());
    }
    let client_ip = client_addr.ip();
    let civ_ssn = fakeit::person::ssn();
    let (civ_id, civ_ref) = app.generate_civ_ref(&civ.first, &civ.last).await;
    let civ_id = civ_id.to_string();

    let new_civ = match 
    sqlx::query_as::<_, Civ>(r#"INSERT INTO civ(id,ref,unique_state_id,first,middle,last,race,date_of_birth,ip_that_created) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *"#)
    .bind(civ_id).bind(civ_ref).bind(civ_ssn).bind(civ.first).bind(civ.middle).bind(civ.last).bind(civ.race).bind(civ.date_of_birth).bind(client_ip).fetch_one(&app.db.0).await {
        Ok(civ) => civ,
        Err(err) => {
            println!("{err:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, json!({"code":"1"}).to_string());
        }
    };

    (StatusCode::OK, serde_json::to_string(&new_civ).unwrap())
}
