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
    /// ブロック
    pub blocks: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: String,
    pub block_id: String,
    pub elements: Vec<Element>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Element {
    #[serde(rename = "rich_text_section")]
    RichTextSection { elements: Vec<Element> },
    #[serde(rename = "rich_text_quote")]
    RichTextQuote { elements: Vec<Element> },
    #[serde(rename = "rich_text_list")]
    RichTextList {
        style: ListStyle,
        indent: i64,
        elements: Vec<Element>,
    },
    #[serde(rename = "rich_text_preformatted")]
    RichTextPreformatted { border: i64, elements: Vec<Element> },
    #[serde(rename = "emoji")]
    Emoji { name: String },
    #[serde(rename = "text")]
    Text { text: String, style: Option<Style> },
    #[serde(rename = "link")]
    Link { url: String, text: Option<String> },
    #[serde(rename = "user")]
    User { user_id: String },
    #[serde(rename = "usergroup")]
    UserGroup { usergroup_id: String },
    #[serde(rename = "broadcast")]
    Broadcast { range: String },
    #[serde(rename = "channel")]
    Channel {
        channel_id: String,
        style: Option<Style>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Style {
    code: Option<bool>,
    bold: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ListStyle {
    #[serde(rename = "bullet")]
    Bullet,
}
