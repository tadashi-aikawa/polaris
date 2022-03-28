use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::domain::entity::Message;
use crate::external::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    messages: Vec<Message>,
}

pub async fn exec(token: &str, query: String) -> Result<Response> {
    let res = slack::SlackClient::new(token)
        .search_message(query.as_str(), "timestamp")
        .await?;
    let messages = res
        .messages
        .matches
        .iter()
        .map(|m| m.into())
        .collect::<Vec<_>>();
    Ok(Response { messages })
}
