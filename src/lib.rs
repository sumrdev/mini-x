pub mod api;
pub mod frontend;
pub mod models;
pub mod schema;

use std::env;

use self::models::*;
use diesel::pg::PgConnection;
use diesel::sql_types::Integer;
use diesel::{prelude::*, sql_query, Connection as Conn};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, username: &str, email: &str, pw_hash: &str) -> Users {
    use self::schema::users;

    let new_post = NewUser {
        username,
        email,
        pw_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_public_messages(conn: &mut PgConnection, limit: i32) -> Vec<(Messages, Users)> {
    use self::schema::messages;
    use self::schema::users;

    messages::table
        .inner_join(users::table.on(messages::author_id.eq(users::user_id)))
        .filter(messages::flagged.eq(0))
        .order_by(messages::pub_date.desc())
        .limit(limit.into())
        .select((Messages::as_select(), Users::as_select()))
        .load(conn)
        .expect("Error loading messages and post")
}

pub fn create_msg(
    conn: &mut PgConnection,
    author_id: &i32,
    text: &str,
    pub_date: String,
    flagged: &i32,
) -> Messages {
    use self::schema::messages;

    let new_message = NewMessage {
        author_id,
        text,
        pub_date: &pub_date,
        flagged,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(Messages::as_select())
        .get_result(conn)
        .expect("Error creating new message")
}

pub fn follow(conn: &mut PgConnection, follower_id: i32, followed_id: i32) {
    use self::schema::followers;

    let new_follower = NewFollower {
        who_id: &follower_id,
        whom_id: &followed_id,
    };

    diesel::insert_into(followers::table)
        .values(&new_follower)
        .returning(Followers::as_select())
        .get_result(conn)
        .expect("Error creating new message");
}

pub fn unfollow(conn: &mut PgConnection, follower_id: i32, followed_id: i32) {
    use self::schema::followers;
    let _ = diesel::delete(
        followers::table.filter(
            followers::who_id
                .eq(follower_id)
                .and(followers::whom_id.eq(followed_id)),
        ),
    )
    .execute(conn);
}

pub fn get_followers(conn: &mut PgConnection, user_id: i32, limit: i32) -> Vec<Users> {
    use self::schema::followers;
    use self::schema::users;

    users::table
        .inner_join(followers::table.on(users::user_id.eq(followers::whom_id)))
        .filter(followers::who_id.eq(user_id))
        .select(Users::as_select())
        .limit(limit.into())
        .load(conn)
        .expect("Couldn't get followers")
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> Option<Users> {
    use self::schema::users;

    users::table
        .find(user_id)
        .select(Users::as_select())
        .first(conn)
        .optional()
        .expect("Error fetching user by id")
}

pub fn get_user_by_name(conn: &mut PgConnection, username: &str) -> Option<Users> {
    use self::schema::users;

    users::table
        .filter(users::username.eq(username))
        .select(Users::as_select())
        .first(conn)
        .optional()
        .expect("Error fetching user by name")
}

pub fn get_user_timeline(conn: &mut PgConnection, id: i32, limit: i32) -> Vec<(Messages, Users)> {
    use self::schema::messages;
    use self::schema::users;

    messages::table
        .inner_join(users::table.on(messages::author_id.eq(users::user_id)))
        .filter(messages::flagged.eq(0))
        .filter(users::user_id.eq(id))
        .order_by(messages::pub_date.desc())
        .limit(limit.into())
        .select((Messages::as_select(), Users::as_select()))
        .load(conn)
        .expect("Error loading messages and post")
}

pub fn get_timeline(conn: &mut PgConnection, id: i32, limit: i32) -> Vec<(Messages, Users)> {
    let query = "((SELECT users.user_id, users.username, users.email, users.pw_hash, 
        messages.message_id, messages.author_id, messages.text, messages.pub_date, messages.flagged 
        FROM followers
        INNER JOIN messages ON followers.whom_id = messages.author_id
        INNER JOIN users ON messages.author_id = users.user_id
        WHERE followers.who_id = $1)
        UNION
        (SELECT users.user_id, users.username, users.email, users.pw_hash, 
        messages.message_id, messages.author_id, messages.text, messages.pub_date, messages.flagged 
        FROM messages
        INNER JOIN users ON messages.author_id = users.user_id
        WHERE users.user_id = $1))
        ORDER BY pub_date DESC
        LIMIT $2;
        ";

    sql_query(query)
        .bind::<Integer, _>(id)
        .bind::<Integer, _>(limit)
        .load::<(Messages, Users)>(conn)
        .expect("")
}

pub fn get_passwd_hash(conn: &mut PgConnection, username: &str) -> Option<String> {
    use self::schema::users;

    users::table
        .filter(users::username.eq(username))
        .select(users::pw_hash)
        .first(conn)
        .optional()
        .expect("Error loading messages and post")
}

pub fn is_following(conn: &mut PgConnection, followed_id: i32, follower_id: i32) -> bool {
    use self::schema::followers;

    let result: Result<Option<i32>, diesel::result::Error> = followers::table
        .find((follower_id, followed_id))
        .select(followers::who_id)
        .first(conn)
        .optional();

    result.unwrap().is_some()
}
