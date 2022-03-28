use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::external::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    real_name: String,
    display_name: String,
    avatar_hash: String,
    image_url: String,
}

impl From<slack::users_profile_get::Profile> for Response {
    fn from(u: slack::users_profile_get::Profile) -> Self {
        Response {
            real_name: u.real_name,
            display_name: u.display_name,
            avatar_hash: u.avatar_hash,
            image_url: u.image_72,
        }
    }
}

pub async fn exec(token: &str) -> Result<Response> {
    let res = slack::SlackClient::new(token)
        .users_profile_get()
        .await?;
    Ok(res.profile.into())
}
