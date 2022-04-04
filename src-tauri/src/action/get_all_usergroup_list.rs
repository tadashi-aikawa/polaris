use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::external::slack;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    usergroup_by_id: HashMap<String, UserGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroup {
    id: String,
    name: String,
}

impl From<slack::usergroups_list::Usergroup> for UserGroup {
    fn from(g: slack::usergroups_list::Usergroup) -> Self {
        UserGroup {
            id: g.id,
            name: g.handle,
        }
    }
}

pub async fn exec(token: &str) -> Result<Response> {
    let res = slack::SlackClient::new(token).usergroups_list().await?;
    let usergroup_by_id =
        res.usergroups
            .into_iter()
            .map(UserGroup::from)
            .fold(HashMap::new(), |mut acc, c| {
                acc.insert(c.id.clone(), c);
                acc
            });
    Ok(Response { usergroup_by_id })
}
