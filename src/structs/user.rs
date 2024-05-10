use serde::{Deserialize, Serialize};
use serde;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: String,
    pub nickname: String,
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    pub signature: String,
    #[serde(rename = "avatarLarger")]
    pub avatar_large: String,
    #[serde(rename = "ftc")]
    pub is_ftc: bool,
    #[serde(rename = "ttSeller")]
    pub is_tiktok_seller: bool,
    #[serde(rename = "secUid")]
    pub sec_uid: String,
    #[serde(rename = "privateAccount")]
    pub is_private: bool,
    #[serde(rename = "secret")]
    pub is_secret: bool,
    #[serde(rename = "verified")]
    pub is_verified: bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "user")]
    pub info: UserInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList {
    #[serde(rename = "userList")]
    pub users: Vec<User>,
    #[serde(rename = "maxCursor")]
    pub max_cursor: isize,
    #[serde(rename = "minCursor")]
    pub min_cursor: isize,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

pub struct FollowersResult {
    pub is_private: bool,
    pub is_secret: bool,
    pub already_checked: bool
}