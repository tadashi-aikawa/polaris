use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub offset: Option<String>,
    pub members: Vec<Member>,
    pub cache_ts: i64,
    pub response_metadata: ResponseMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub deleted: bool,
    pub color: Option<String>,
    pub real_name: Option<String>,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<i64>,
    pub profile: Profile,
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub is_bot: bool,
    pub is_app_user: bool,
    pub updated: i64,
    pub is_email_confirmed: Option<bool>,
    pub who_can_share_contact_card: Option<WhoCanShareContactCard>,
    pub has_2_fa: Option<bool>,
    pub is_workflow_bot: Option<bool>,
    pub is_invited_user: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub title: String,
    pub phone: String,
    pub skype: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub display_name: String,
    pub display_name_normalized: String,
    pub fields: Option<serde_json::Value>,
    pub status_text: String,
    pub status_emoji: String,
    pub status_emoji_display_info: Vec<StatusEmojiDisplayInfo>,
    pub status_expiration: i64,
    pub avatar_hash: String,
    pub always_active: Option<bool>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String,
    pub image_512: String,
    pub image_1024: Option<String>,
    pub status_text_canonical: String,
    pub team: String,
    pub image_original: Option<String>,
    pub is_custom_image: Option<bool>,
    pub huddle_state: Option<String>,
    pub huddle_state_expiration_ts: Option<i64>,
    pub guest_invited_by: Option<String>,
    pub api_app_id: Option<String>,
    pub bot_id: Option<String>,
    pub who_can_share_contact_card: Option<WhoCanShareContactCard>,
}

#[derive(Serialize, Deserialize)]
pub struct StatusEmojiDisplayInfo {
    pub emoji_name: String,
    pub display_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}

#[derive(Serialize, Deserialize)]
pub enum WhoCanShareContactCard {
    #[serde(rename = "EVERYONE")]
    Everyone,

    #[serde(rename = "NO_ONE")]
    NoOne,

    #[serde(rename = "TEAM_MEMBERS")]
    TeamMembers,
}
