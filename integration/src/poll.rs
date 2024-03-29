use std::str::FromStr;

use chrono::Utc;
use futures::StreamExt;
use notion::{
    ids::DatabaseId,
    models::search::{DatabaseQuery, DateCondition, FilterCondition, TimestampCondition},
    NotionApi,
};
use sqlx::PgPool;
use tokio::time::interval;

use crate::{
    shared::{get_client, get_polling_interval},
    HOOK_URL,
};

#[derive(sqlx::FromRow, Debug)]
struct Poll {
    token: String,
    database: String,
}

pub async fn poll(pool: &PgPool) {
    let dura = get_polling_interval().to_std().unwrap();
    let mut poll_interval = interval(dura);

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
                post_message(token, database).await;
            }
        }
    }
}

async fn post_message(token: String, database: String) {
    let now = Utc::now();
    let dura = get_polling_interval();
    let world_before_dura = now - *dura;

    let notion = NotionApi::new(token).unwrap();

    let query = DatabaseQuery {
        sorts: None,
        filter: Some(FilterCondition::Timestamp {
            timestamp: "last_edited_time".to_string(),
            condition: TimestampCondition::LastEditedTime(DateCondition::OnOrAfter(
                world_before_dura,
            )),
        }),
        paging: None,
    };
    let pages = notion
        .query_database(DatabaseId::from_str(&database).unwrap(), query)
        .await;

    if let Ok(page_list) = pages {
        let client = get_client();

        for page in page_list.results() {
            _ = client.post(HOOK_URL.as_str()).json(&page).send().await;
        }
    }
}
