use std::sync::Mutex;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

pub struct LatestAction{
    pub latest: Mutex<i32>
}

#[derive(Serialize, Deserialize)]
pub struct Latest{
    pub latest: i32
}
impl Default for Latest {
    fn default() -> Self {
        Latest {
            latest: -1
        }
    }
}

#[derive(Deserialize)]
pub struct MessageAmount{
    pub no: i32
}
impl Default for MessageAmount {
    fn default() -> Self {
        MessageAmount {
            no: 100
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub pwd: String,
}

#[derive(Serialize)]
pub struct RegisterError {
    pub status: i32,
    pub error_msg: String
}

#[derive(Serialize)]
pub struct Message {
    pub content: String,
    pub user: String,
    #[serde(with = "ts_seconds")]
    pub pub_date: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct MessageContent {
    pub content: String
}

#[derive(Serialize)]
pub struct Follows {
    pub follows: Vec<String>
}

#[derive(Deserialize)]
pub struct FollowParam {
    pub follow: Option<String>,
    pub unfollow: Option<String>
}