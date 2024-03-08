use askama_actix::Template;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String
}

#[derive(Debug)]
pub struct Messages {
    pub text: String,
    pub username: String,
    pub pub_date: DateTime<Utc>,
    pub gravatar_url: String
}

#[derive(Template)]
#[template(path = "../templates/timeline.html")]
pub struct TimelineTemplate<'a> {
    pub messages: Vec<Messages>, 
    pub user: Option<User>,
    pub request_endpoint: &'a str, 
    pub profile_user: Option<User>,
    pub followed: Option<bool>, 
    pub flashes: Vec<String>,
    pub title: String
}

#[derive(Template)]
#[template(path = "../templates/login.html")]
pub struct LoginTemplate {
    pub user: Option<User>,
    pub error: String,
    pub flashes: Vec<String>,
    pub username: String, 
}

#[derive(Template)]
#[template(path = "../templates/register.html")]
pub struct RegisterTemplate {
    pub user: Option<User>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub flashes: Vec<String>,
    pub error: String,
}

#[derive(Deserialize)]
pub struct MessageInfo {
    pub text: String,
}

#[derive(Deserialize)]
pub struct LoginInfo {
   pub username: String,
   pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password2: String,
}
