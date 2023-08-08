use std::env;

use notion_flows::{listen_to_event, notion::models::Page};
use tg_flows::{ChatId, Telegram};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let database = env::var("database").unwrap();
    let token = env::var("token").unwrap();
    let chat_id = env::var("chat_id").unwrap();

    let chat_id = ChatId(chat_id.parse().unwrap());
    let tele = Telegram::new(token);

    let send = |msg: String| {
        tele.send_message(chat_id, msg).ok();
    };

    listen_to_event(database, |page| async { handler(page, send).await }).await;
}

async fn handler<F>(page: Page, send: F)
where
    F: Fn(String),
{
    let title = page.title().unwrap_or("<untitled>".to_string());
    let pros: String = page
        .properties
        .properties
        .iter()
        .map(|(k, v)| format!("- {k}: {v:?}"))
        .collect();

    let msg = format!("# {title}\n{pros}");
    send(msg);
}
