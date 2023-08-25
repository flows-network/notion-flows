use axum::extract::{Path, Query, State};
use reqwest::StatusCode;

use crate::{
    model::database::{Flow, ListenerQuery},
    state::AppState,
};

pub async fn listen(
    Path(Flow {
        flows_user,
        flow_id,
        handler_fn: _,
    }): Path<Flow>,
    State(AppState { pool }): State<AppState>,
    Query(ListenerQuery {
        database,
        handler_fn,
    }): Query<ListenerQuery>,
) -> Result<StatusCode, String> {
    // TODO: check if the permissions can access the database

    let sql = "
        INSERT INTO listener(flow_id, flows_user, handler_fn, database)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (flow_id, flows_user, database)
        DO UPDATE SET handler_fn = excluded.handler_fn
    ";
    sqlx::query(sql)
        .bind(flow_id)
        .bind(flows_user)
        .bind(handler_fn)
        .bind(database)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(StatusCode::OK)
}
