pub mod notion {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct DatabaseQuery {
        pub results: Vec<Result>,
        pub next_cursor: Option<String>,
        pub has_more: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Result {
        pub object: String,
        pub id: String,
        pub created_time: String,
        pub last_edited_time: String,
        pub created_by: By,
        pub last_edited_by: By,
        pub cover: Value,
        pub icon: Value,
        pub parent: Parent,
        pub archived: bool,
        pub properties: Value,
        pub url: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct By {
        pub object: String,
        pub id: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Parent {
        #[serde(rename = "type")]
        pub type_field: String,
        pub database_id: String,
    }
}

pub mod database {
    use serde::Deserialize;

    #[derive(Debug, Deserialize, sqlx::FromRow)]
    pub struct Flow {
        pub flows_user: String,
        pub flow_id: String,
        pub handler_fn: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct Access {
        pub code: String,
        pub state: String,
    }

    #[derive(Deserialize)]
    pub struct Token {
        pub access_token: String,
        pub bot_id: String,
        // duplicated_template_id: Option<String>,
        // owner: (),
        // workspace_icon: Option<String>,
        pub workspace_id: String,
        pub workspace_name: Option<String>,
    }

    #[derive(sqlx::FromRow)]
    pub struct Workspace {
        #[sqlx(rename = "workspace_id")]
        pub id: String,
        #[sqlx(rename = "workspace_name")]
        pub name: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct ListenerQuery {
        pub database: String,
        pub handler_fn: Option<String>,
    }
}
