use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};

use crate::{
    get_client,
    model::database::{Access, Token},
    state::AppState,
    CLIENT_ID, CLIENT_SECRET, REDIRECT_URL,
};

pub async fn auth(
    Query(access): Query<Access>,
    State(AppState { pool }): State<AppState>,
) -> impl IntoResponse {
    let code = access.code;
    let flows_user = access.state;

    let url = "https://api.notion.com/v1/oauth/token";
    let client = get_client();

    let resp = client
        .post(url)
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .json(&serde_json::json!({
            "grant_type": "authorization_code",
            "code": code,
            "redirect_uri": "https://flows.purejs.icu/auth",
        }))
        .send()
        .await;

    match resp {
        Ok(r) => {
            let text = r.text().await.map_err(|e| e.to_string())?;
            let json: Token =
                serde_json::from_str(&text).map_err(|e| format!("raw: {text}, error: {e}"))?;

            let bot_id = json.bot_id;
            let token = json.access_token;
            let workspace_id = json.workspace_id;
            let workspace_name = json.workspace_name;

            let sql = "
                INSERT INTO bot(bot_id, flows_user, token, workspace_id, workspace_name)
                VALUES ($1, $2, $3, $4, $5)
            ";
            let _query_result = sqlx::query(sql)
                .bind(bot_id)
                .bind(flows_user)
                .bind(token.trim())
                .bind(workspace_id)
                .bind(workspace_name)
                .execute(&*pool)
                .await
                .map_err(|e| e.to_string())?;

            Ok(Redirect::to(REDIRECT_URL))
        }
        Err(e) => Err(e.to_string()),
    }
}
