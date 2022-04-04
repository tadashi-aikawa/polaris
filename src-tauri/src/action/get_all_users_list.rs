use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::external::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    user_by_id: HashMap<String, User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    real_name: Option<String>,
    images: Images,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Images {
    image_32: String,
    image_48: String,
    image_512: String,
}

impl From<slack::users_list::Member> for User {
    fn from(r: slack::users_list::Member) -> Self {
        User {
            id: r.id,
            name: r.name,
            real_name: r.real_name,
            images: Images {
                image_32: r.profile.image_32,
                image_48: r.profile.image_48,
                image_512: r.profile.image_512,
            },
        }
    }
}

pub async fn exec(token: &str) -> Result<Response> {
    let mut cursor = "".to_string();
    let mut users = Vec::<User>::new();

    loop {
        let res = slack::SlackClient::new(token).users_list(&cursor).await?;
        let _users = res.members.into_iter().map(User::from).collect::<Vec<_>>();
        users.extend(_users);

        cursor = res.response_metadata.next_cursor;
        println!("{:?}", cursor);
        if cursor.is_empty() {
            break;
        }
    }

    let user_by_id: HashMap<String, User> = users.into_iter().fold(HashMap::new(), |mut acc, c| {
        acc.insert(c.id.clone(), c);
        acc
    });

    Ok(Response { user_by_id })
}
