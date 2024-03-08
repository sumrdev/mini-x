pub mod api;
pub mod frontend;
pub mod models;
pub mod schema;

use self::models::{NewUser, User};
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
