use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub usergroups: Vec<Usergroup>,
}

#[derive(Serialize, Deserialize)]
pub struct Usergroup {
    pub id: String,
    pub team_id: String,
    pub is_usergroup: bool,
    pub is_subteam: bool,
    pub name: String,
    pub description: String,
    pub handle: String,
    pub is_external: bool,
    pub date_create: i64,
    pub date_update: i64,
    pub date_delete: i64,
    pub enterprise_subteam_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub prefs: Prefs,
    pub user_count: i64,
    pub channel_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Prefs {
    pub channels: Vec<String>,
    pub groups: Vec<String>,
}
