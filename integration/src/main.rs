use crate::route::static_path;
use crate::route::{auth, connected, event, listen, revoke};

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use include_dir::{include_dir, Dir};
use shared::get_client;

use poll::poll;
use sqlx::{Executor, PgPool};
use state::AppState;

mod model;
mod poll;
mod route;
mod shared;
mod state;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

const POLLING_INTERVAL_SECS: i64 = 60;
const POST_INTERVAL_SECS: u64 = 1;

lazy_static::lazy_static! {
    static ref HOOK_URL: String =
        std::env::var("PLATFORM_HOOK_URL").unwrap_or(String::from("https://code.flows.network/hook/notion/message"));
    static ref CLIENT_ID: String = std::env::var("NOTION_APP_CLIENT_ID").unwrap();
    static ref CLIENT_SECRET: String = std::env::var("NOTION_APP_CLIENT_SECRET").unwrap();
    static ref REDIRECT_URL: String = std::env::var("NOTION_OAUTH_REDIRECT_URL").unwrap();
}

#[tokio::main]
async fn main() {
    let state = init().await;

    let app = Router::new()
        .route("/:flows_user/:flow_id/listen", post(listen))
        .route("/:flows_user/:flow_id/revoke", post(revoke))
        .route("/event/:database", get(event))
        .route("/connected/:flows_user", get(connected))
        .route("/auth", get(auth))
        .route("/static/*path", get(static_path))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:6970".parse().unwrap())
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
