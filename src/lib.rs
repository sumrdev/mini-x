pub mod api;
pub mod frontend;
pub mod models;
pub mod schema;
use self::models::*;
use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, Connection as Conn};
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "/tmp/mini-x.db";

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(
    conn: &mut SqliteConnection,
    username: &str,
    email: &str,
    pw_hash: &str,
) -> User {
    use self::schema::user;

    let new_post = NewUser {
        username,
        email,
        pw_hash,
    };

    diesel::insert_into(user::table)
        .values(&new_post)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_public_messages(conn: &mut SqliteConnection, limit: i32) -> Vec<(Message, User)> {
    use self::schema::message;
    use self::schema::user;

    message::table
        .inner_join(user::table.on(message::author_id.eq(user::user_id)))
        .filter(message::flagged.eq(0))
        .order_by(message::pub_date.desc())
        .limit(limit.into())
        .select((Message::as_select(), User::as_select()))
        .load(conn)
        .expect("Error loading messages and post")
}

pub fn create_msg(conn: &mut SqliteConnection, author_id: &i32, text: &str, pub_date: String, flagged: &i32) -> Message {
    use self::schema::message;

    let new_message = NewMessage {
        author_id,
        text,
        pub_date: &pub_date,
        flagged
    };

    diesel::insert_into(message::table)
        .values(&new_message)
        .returning(Message::as_select())
        .get_result(conn)
        .expect("Error creating new message")

}

pub fn follow(conn: &mut SqliteConnection, follower_id: i32, followed_id: i32) {
    use self::schema::follower;

    let new_follower = NewFollower {
        who_id: &follower_id,
        whom_id: &followed_id,
    };
    
    diesel::insert_into(follower::table)
        .values(&new_follower)
        .returning(Follower::as_select())
        .get_result(conn)
        .expect("Error creating new message");
}

pub fn unfollow(conn: &mut SqliteConnection, follower_id: i32, followed_id: i32) {
    use self::schema::follower;
    let _ = diesel::delete(
        follower::table.filter(
            follower::who_id.eq(follower_id)
            .and(follower::whom_id.eq(followed_id))))
            .execute(conn);
}

pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: i32) -> Option<User> {
    use self::schema::user;

    user::table
        .find(user_id)
        .select(User::as_select())
        .first(conn)
        .optional()
        .expect("Error fetching user by id")
}

pub fn get_user_by_name(conn: &mut SqliteConnection, username: &str) -> Option<User> {
    use self::schema::user;

    user::table
        .filter(user::username.eq(username))
        .select(User::as_select())
        .first(conn)
        .optional()
        .expect("Error fetching user by name")
}

pub fn get_user_timeline(conn: &mut SqliteConnection, id: i32, limit: i32) -> Vec<(Message, User)> {
    use self::schema::message;
    use self::schema::user;

    message::table
        .inner_join(user::table.on(message::author_id.eq(user::user_id)))
        .filter(message::flagged.eq(0))
        .filter(user::user_id.eq(id))
        .order_by(message::pub_date.desc())
        .limit(limit.into())
        .select((Message::as_select(), User::as_select()))
        .load(conn)
        .expect("Error loading messages and post")
}

// pub fn get_timeline(conn: &mut SqliteConnection, id: i32, limit: i32) -> Vec<(Message, User)> {
//     use self::schema::follower;
//     use self::schema::message;
//     use self::schema::user;

//     message::table
//         .inner_join(user::table.on(message::author_id.eq(user::user_id)))
//             .filter(
//                 message::flagged.eq(0)
//                 .and(
//                     user::user_id.eq(id)
//                         .or(
//                             follower::table
//                             .filter(
//                                 follower::whom_id.eq(id))
//                                 .select(follower::who_id)),
//             ),
//         )
//         .limit(limit.into())
//         .select((Message::as_select(), User::as_select()))
//         .order_by(message::pub_date.desc())
//         .load(conn)
//         .expect("Error loading messages and post")
// }

pub fn get_passwd_hash(conn: &mut SqliteConnection, username: &str) -> Option<String> {
    use self::schema::user;

    user::table
        .filter(user::username.eq(username))
        .select(user::pw_hash)
        .first(conn)
        .optional()
        .expect("Error loading messages and post")
}
