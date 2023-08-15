use crate::{CLIENT_ID, REDIRECT_URL};
use axum::{extract::Path, response::Redirect};

pub async fn access(Path(flows_user): Path<String>) -> Redirect {
    let url = format!("https://api.notion.com/v1/oauth/authorize?client_id={}&response_type=code&owner=user&state={}&redirect_uri={}", CLIENT_ID.as_str(), flows_user, REDIRECT_URL.as_str());
    Redirect::to(&url)
}
