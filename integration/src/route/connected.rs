use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::Value;

use crate::{model::database::Workspace, state::AppState};

pub async fn connected(
    Path(flows_user): Path<String>,
    State(AppState { pool }): State<AppState>,
) -> Result<Json<Value>, String> {
    let mut results = Vec::new();

    let sql = "SELECT workspace_id, workspace_name FROM bot WHERE flows_user = $1";
    let Workspace { id, name } = sqlx::query_as(sql)
        .bind(flows_user)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;

    let name = if let Some(n) = name {
        format!("({n})")
    } else {
        String::new()
    };
    let display = id + &name;
    results.push(serde_json::json!({
        "name": display,
    }));

    Ok(Json(serde_json::json!({
        "title": "Connected workspace",
        "list": results,
    })))
}
