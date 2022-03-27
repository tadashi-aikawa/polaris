use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::external::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    id: String,
    user_id: String,
    user_name: String,
    channel_name: String,
    text: String,
    permalink: String,
    created_at: DateTime<Utc>,
}

impl From<&slack::search_messages::Message> for Message {
    fn from(m: &slack::search_messages::Message) -> Self {
        let sec = m.ts.split('.').collect::<Vec<_>>()[0];
        let sec = sec.parse::<i64>().unwrap();

        Message {
            id: m.iid.clone(),
            user_id: m.user.clone(),
            user_name: m.username.clone(),
            channel_name: m.channel.name.clone(),
            text: m.text.clone(),
            permalink: m.permalink.clone(),
            created_at: DateTime::from_utc(NaiveDateTime::from_timestamp(sec, 0), Utc),
        }
    }
}

pub async fn exec(query: String) -> Result<Response> {
    let token =
        std::env::var("POLARIS_SLACK_USER_TOKEN").expect("POLARIS_SLACK_USER_TOKEN is not set.");
    let res = slack::SlackClient::new(token.as_str())
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
