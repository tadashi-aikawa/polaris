use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub emoji: HashMap<String, String>,
    pub cache_ts: String,
}
