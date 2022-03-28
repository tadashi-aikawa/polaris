pub mod search_messages;
pub mod users_profile_get;

use anyhow::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct SlackClient {
    base_url: String,
    token: String,
}

impl SlackClient {
    pub fn new(token: &str) -> SlackClient {
        SlackClient {
            base_url: "https://slack.com/api".into(),
            token: token.into(),
        }
    }

    pub async fn search_message(&self, query: &str, sort: &str) -> Result<search_messages::Response, Error> {
        self.get_request("/search.messages", &[("query", query), ("sort", sort)]).await
    }

    pub async fn users_profile_get(&self) -> Result<users_profile_get::Response, Error> {
        self.get_request("/users.profile.get", &[("", "")]).await
    }

    async fn get_request<T: Serialize + ?Sized, R: DeserializeOwned>(&self, path: &str, query: &T) -> Result<R, Error> {
        Ok(reqwest::Client::new()
            .get(format!("{}{}", self.base_url, path))
            .query(query)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<R>()
            .await?)
    }
}
