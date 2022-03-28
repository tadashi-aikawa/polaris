use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::domain::entity::User;

use crate::external::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    user: User
}

pub async fn exec(token: &str) -> Result<Response> {
    let res = slack::SlackClient::new(token)
        .users_profile_get()
        .await?;
    Ok(Response {user: res.profile.into()})
}
