use axum::extract::{Path, Query, State};
use reqwest::StatusCode;

use crate::{
    model::database::{Flow, ListenerQuery},
    state::AppState,
};

pub async fn revoke(
    Path(Flow {
        flows_user,
        flow_id,
    }): Path<Flow>,
    State(AppState { pool }): State<AppState>,
    Query(ListenerQuery { database }): Query<ListenerQuery>,
) -> Result<StatusCode, String> {
    let sql = "
        DELETE FROM listener
        WHERE flow_id = $1 AND flows_user = $2 AND database = $3
    ";
    sqlx::query(sql)
        .bind(flow_id)
        .bind(flows_user)
        .bind(database)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(StatusCode::OK)
}
