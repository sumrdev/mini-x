use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Latest{
    pub latest: i32
}

pub struct LatestAction{
    pub latest: Mutex<i32>
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