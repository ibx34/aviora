mod app;
mod config;
mod db;
mod models;
mod routes;

use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use routes::civs::register_new_civ;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .pretty()
        .init();

    let app: app::App = app::App::init().await?;
    app.db.migrate().await?;

    let civ_routes = Router::new().route("/register", post(register_new_civ));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let app = Router::new().nest("/civ", civ_routes).with_state(app);

    println!("Starting on: {addr:?}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}
