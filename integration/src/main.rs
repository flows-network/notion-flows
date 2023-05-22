use crate::route::{auth, connected, event, listen, revoke};

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use shared::get_client;

use poll::poll;
use sqlx::{Executor, PgPool};
use state::AppState;

mod model;
mod poll;
mod route;
mod shared;
mod state;

const HOOK_URL: &str = "https://code.flows.network/hook/notion/message";

const REDIRECT_URL: &str = "https://flows.network";
const CLIENT_ID: &str = "1025ce97-c5dc-4c37-bba6-fe4801db5e0e";

const POLLING_INTERVAL_SECS: i64 = 100;

const CLIENT_SECRET: &str = env!("CLIENT_SECRET");

#[tokio::main]
async fn main() {
    let state = init().await;

    let app = Router::new()
        .route("/:flows_user/:flow_id/listen", post(listen))
        .route("/:flows_user/:flow_id/revoke", post(revoke))
        .route("/event/:database", get(event))
        .route("/connected/:flows_user", get(connected))
        .route("/auth", get(auth))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:6870".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn init() -> AppState {
    #[cfg(feature = "debug")]
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = Arc::new(PgPool::connect(&db_url).await.unwrap());

    pool.execute(include_str!("../schema.sql")).await.unwrap();

    let s_pool = pool.clone();
    tokio::spawn(async move {
        poll(&s_pool).await;
    });

    AppState { pool }
}
