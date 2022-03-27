use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub messages: Messages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub total: i32,
    pagination: Pagination,
    pub matches: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    /// ex: 145253
    total_count: i32,
    /// ex: 1
    page: i32,
    /// ex: 20
    per_page: i32,
    /// ex: 7263
    page_count: i32,
    /// ex: 1
    first: i32,
    /// ex: 20
    last: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    /// ex: 123456789-1234-1234-abcd-123456abcd78
    pub iid: String,
    /// ex: 123.45678
    pub score: f64,
    /// ex: U1ABCDE3
    pub user: String,
    /// ex: tadashi-aikawa
    pub username: String,
    /// ex: 1647445574.312239
    pub ts: String,
    /// channel information
    pub channel: Channel,
    /// ex: 本文です
    pub text: String,
    /// ex: https://hoge.slack.com/archives/U1ABCDE3/p123456789012345678
    pub permalink: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
}
