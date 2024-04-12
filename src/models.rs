use crate::schema::{followers, messages};
use diesel::prelude::*;

use super::schema::users;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(QueryableByName)]
pub struct Users {
    pub user_id: i32,
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
#[derive(QueryableByName)]
pub struct Followers {
    pub who_id: i32,
    pub whom_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = followers)]
pub struct NewFollower<'a> {
    pub who_id: &'a i32,
    pub whom_id: &'a i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(QueryableByName)]
pub struct Messages {
    pub message_id: i32,
    pub author_id: i32,
    pub text: String,
    pub pub_date: String,
    pub flagged: i32,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub author_id: &'a i32,
    pub text: &'a str,
    pub pub_date: &'a String,
    pub flagged: &'a i32,
}
