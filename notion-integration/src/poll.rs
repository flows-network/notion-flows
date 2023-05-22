use chrono::{Duration, Utc};
use futures::StreamExt;
use sqlx::PgPool;
use tokio::time::interval;

use crate::{
    model::notion::DatabaseQuery,
    shared::{get_client, get_polling_interval},
    HOOK_URL,
};

#[derive(sqlx::FromRow)]
struct Poll {
    token: String,
    database: String,
}

pub async fn poll(pool: &PgPool) {
    let dura = get_polling_interval().to_std().unwrap();
    let mut poll_interval = interval(dura);

    let mut post_interval = interval(std::time::Duration::from_secs(3));

    loop {
        poll_interval.tick().await;

        let select_token = "
            WITH listening AS (
                SELECT
                    flows_user,
                    database
                FROM
                    listener
            )

            SELECT
                bot.token,
                listening.database
            FROM bot
            INNER JOIN listening ON bot.flows_user = listening.flows_user;
        ";
        let mut stream = sqlx::query_as::<_, Poll>(select_token).fetch(pool);

        while let Some(res) = stream.next().await {
            if let Ok(Poll { token, database }) = res {
                post_interval.tick().await;
                post_message(token, database).await;
            }
        }
    }
}

async fn post_message(token: String, database: String) {
    let now = Utc::now();
    let world_before_100s = now - Duration::seconds(100);
    let formatted = format!("{}", world_before_100s.format("%Y-%m-%dT%H:%M:%S"));

    let client = get_client();

    let json = serde_json::json!({
        "filter": {
            "timestamp": "created_time",
            "created_time": {
                "on_or_after": formatted
            }
        }
    });

    let url = format!("https://api.notion.com/v1/databases/{database}/query");
    let result = client
        .post(url)
        .bearer_auth(token)
        .header("Notion-Version", "2022-06-28")
        .json(&json)
        .send()
        .await;

    if let Ok(resp) = result {
        let text = resp.text().await.map_err(|e| e.to_string()).unwrap();
        let dq: DatabaseQuery = serde_json::from_str(&text)
            .map_err(|e| format!("raw: {text}, error: {e}"))
            .unwrap();

        for rst in dq.results {
            _ = client.post(HOOK_URL).json(&rst).send().await;
        }
    }
}
