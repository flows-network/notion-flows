<div align="center">
  <h1><code>Notion Flows</code></h1>
  <a href="https://docs.rs/notion-flows/">
    <img src="https://docs.rs/notion-flows/badge.svg">
  </a>
  <a href="https://crates.io/crates/notion-flows">
    <img src="https://img.shields.io/crates/v/notion-flows.svg">
  </a>

  Notion Integration for [Flows.network](https://flows.network)
</div>

## Quick Start

### Listen

This is a changed database forward (to telegram) bot.

```rust
use std::env;

use notion_flows::{database_update_handler, listen_to_database_update, notion::models::Page};
use tg_flows::{ChatId, Telegram};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let database = env::var("database").unwrap();

    listen_to_database_update(database).await;
}

#[database_update_handler]
async fn handler(page: Page) {
    let title = page.title().unwrap_or("<untitled>".to_string());
    let pros: String = page
        .properties
        .properties
        .iter()
        .map(|(k, v)| format!("- {}: {:?}", k, v))
        .collect();

    let msg = format!("# {}\n{}", title, pros);

    let chat_id = env::var("chat_id").unwrap();
    let chat_id = ChatId(chat_id.parse().unwrap());

    let token = env::var("token").unwrap();
    let tele = Telegram::new(token);
    tele.send_message(chat_id, msg).ok();
}
```

[listen_to_database_update()] is responsible for registering a listener for the database.
When a new `database` changes (as `page`) coming, the `handler` is called with received `Page`.

### Action

```rust
    use std::str::FromStr;
    use notion_flows::notion::{ids::PageId, NotionApi};

    let notion = NotionApi::new("API_TOKEN").unwrap();
    let page_id = PageId::from_str("PAGE_ID").unwrap();
    let page = notion.get_page(page_id).await;
```

Action as [NotionApi](https://docs.rs/notion-wasi/0.5.2/notion_wasi/struct.NotionApi.html).

ref: [Notion API Reference](https://developers.notion.com/reference/intro)
