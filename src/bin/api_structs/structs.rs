use std::sync::Mutex;

use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Latest{
    pub latest: i32
}

pub struct LatestAction{
    pub latest: Mutex<i32>
}

#[derive(Deserialize)]
pub struct MessagesQuery{
    pub no: i32
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