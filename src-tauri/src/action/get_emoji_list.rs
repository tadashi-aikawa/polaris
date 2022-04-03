use anyhow::Result;

use crate::external::slack;

pub type Response = slack::emoji_list::Response;

pub async fn exec(token: &str) -> Result<Response> {
    let res = slack::SlackClient::new(token)
        .emoji_list()
        .await?;
    Ok(res)
}
