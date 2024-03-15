use crate::schema::{followers, messages};
use diesel::prelude::*;

use super::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub pw_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub pw_hash: &'a str,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::followers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Followers {
    pub who_id: i64,
    pub whom_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = followers)]
pub struct NewFollower<'a> {
    pub who_id: &'a i64,
    pub whom_id: &'a i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Messages {
    pub message_id: i64,
    pub author_id: i64,
    pub text: String,
    pub pub_date: String,
    pub flagged: i64,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub author_id: &'a i64,
    pub text: &'a str,
    pub pub_date: &'a String,
    pub flagged: &'a i64,
}
