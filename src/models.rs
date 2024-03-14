use crate::schema::{follower, message};
use diesel::prelude::*;

use super::schema::user;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub pw_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub pw_hash: &'a str,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::follower)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Follower {
    pub who_id: i32,
    pub whom_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = follower)]
pub struct NewFollower<'a> {
    pub who_id: &'a i32,
    pub whom_id: &'a i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub message_id: i32,
    pub author_id: i32,
    pub text: String,
    pub pub_date: String,
    pub flagged: i32,
}

#[derive(Insertable)]
#[diesel(table_name = message)]
pub struct NewMessage<'a> {
    pub author_id: &'a i32,
    pub text: &'a str,
    pub pub_date: &'a String,
    pub flagged: &'a i32,
}
