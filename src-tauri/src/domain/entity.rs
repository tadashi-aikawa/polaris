use crate::external::slack;
use crate::external::slack::search_messages::{Attachment, Block};
use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub real_name: String,
    pub display_name: String,
    pub avatar_hash: String,
    pub image_url: String,
}

impl From<slack::users_profile_get::Profile> for User {
    fn from(u: slack::users_profile_get::Profile) -> Self {
        User {
            real_name: u.real_name,
            display_name: u.display_name,
            avatar_hash: u.avatar_hash,
            image_url: u.image_72,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub user_id: Option<String>,
    pub user_name: String,
    pub channel_name: String,
    pub text: String,
    pub permalink: String,
    pub created_at: DateTime<Local>,
    pub attachments: Option<Vec<Attachment>>,
    pub blocks: Option<Vec<Block>>,
}

impl From<&slack::search_messages::Message> for Message {
    fn from(m: &slack::search_messages::Message) -> Self {
        let sec = m.ts.split('.').collect::<Vec<_>>()[0];
        let sec = sec.parse::<i64>().unwrap();

        Message {
            id: format!("{}/{}", m.channel.id, m.ts),
            user_id: m.user.clone(),
            user_name: m.username.clone(),
            channel_name: m.channel.name.clone(),
            text: m.text.clone(),
            permalink: m.permalink.clone(),
            created_at: Local.timestamp(sec, 0),
            attachments: m.attachments.clone(),
            blocks: m.blocks.clone(),
        }
    }
}
