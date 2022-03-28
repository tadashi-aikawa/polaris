use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    /// I am hogehoge
    pub title: String,
    /// Tadashi Aikawa
    pub real_name: String,
    /// Tadashi Aikawa
    pub real_name_normalized: String,
    /// tadashi-aikawa
    pub display_name: String,
    /// tadashi-aikawa
    pub display_name_normalized: String,
    /// Out of Office
    pub status_text: String,
    /// :no_entry:
    pub status_emoji: String,
    /// abcdefghi012
    pub avatar_hash: String,
    /// https://.....png
    pub image_original: String,
    /// "tadashi"
    pub first_name: String,
    /// "aikawa"
    pub last_name: String,
    /// https://.....png
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String,
    pub image_512: String,
    pub image_1024: String,
}
