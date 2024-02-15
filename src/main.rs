use actix_files as fs;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::web::{self, Redirect};
use actix_identity::IdentityMiddleware;
use actix_identity::Identity;

use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::{cookie::Key, get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use actix_web_flash_messages::{FlashMessage, IncomingFlashMessages};
use askama_actix::Template;
use chrono::{DateTime, Utc};
use md5::{Digest, Md5};
use rusqlite::{params, Connection, Result};
use serde::Deserialize;
use uuid::Uuid;
use pwhash::bcrypt;

#[derive(Template)] // this will generate the code...
#[template(path = "../templates/hello.html")] // using the template in this path, relative
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
struct User {
    user_id: Uuid,
    username: String,
}
struct G {
    db: Connection,
    user: User,
}
struct Messages {
    text: String,
    email: String,
    username: String,
    pub_date: DateTime<Utc>,
    gravatar_url: String
}
// https://doc.rust-lang.org/std/vec/index.html
// https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Template)]
#[template(path = "../templates/timeline.html")]
struct TimelineTemplate<'a> {
    messages: Vec<Messages>, // Vec<Message>, dynamic array of message structs
    user: Option<User>,
    request_endpoint: &'a str, //just an URL does not need to be strict
    profile_user: Option<User>,
    followed: Option<bool>, //Unsure how to define this properly
    flashes: Vec<String>,
    title: &'a str
}

#[derive(Template)]
#[template(path = "../templates/login.html")]
struct LoginTemplate {
    user: Option<User>,
    //g: Option<G>,
    flashes: Vec<String>,
    username: String, // Is it not title
    error: String,    // Is it not title
}

#[derive(Template)]
#[template(path = "../templates/register.html")]
struct RegisterTemplate {
    user: Option<User>,
    email: String,
    username: String,
    password: String,
    flashes: Vec<String>,
    error: String,    // Is it not title
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let signing_key = Key::generate(); // This will usually come from configuration!
    let message_store = CookieMessageStore::builder(signing_key).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let _ = init_db();
    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .service(fs::Files::new("/static", "./static/").index_file("index.html"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64])) //Maybe not 64 zeros as key
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(message_framework.clone())
            .service(register)
            .service(post_register)
            .service(timeline)
            .service(login)
            .service(post_login)
            .service(logout)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn get_database_string() -> String {
    String::from("/tmp/mini-x.db")
}

fn connect_db() -> Connection {
    Connection::open(get_database_string()).unwrap()
}

fn init_db() -> rusqlite::Result<()> {
    let schema_sql = std::fs::read_to_string("schema.sql").unwrap();
    let conn = connect_db();

    conn.execute_batch(&schema_sql)?;
    Ok(())
}

fn query_db(query: &str) {}

fn get_user_id(username: &str) -> Result<usize, rusqlite::Error> {
    let conn = connect_db();
    conn.execute(
        "SELECT user_id FROM user WHERE username = ?1",
        params![username],
    )
}

fn g(session: Session) -> Result<()> {
    let connection = connect_db();
    /*   if let Some(user_id) = session.get::<Uuid>("user_id")? {

    } else {

    } */
    Ok(())
}

fn g_mock() -> Result<G> {
    let connection = Connection::open_in_memory()?;

    Ok(G {
        db: connection,
        user: User {
            user_id: Uuid::new_v4(),
            username: String::from("Test Name"),
        },
    })
}

fn gravatar_url(email: &str) -> String {
    let hash = Md5::digest(email.trim().to_lowercase().as_bytes());
    
    let hash_str = format!("{:x}", hash);
    
    format!("https://www.gravatar.com/avatar/{}?d=identicon&s={}", hash_str, 48)
}

fn get_messages() -> Vec<Messages> {
    vec![
        Messages {
            text: String::from("Hello, world!"),
            email: String::from("example@email.com"),
            username: String::from("user123"),
            pub_date: Utc::now(),
            gravatar_url: String::from("")
        },
        Messages {
            text: String::from("How 
            are you?"),
            email: String::from("another@email.com"),
            username: String::from("user456"),
            pub_date: Utc::now(),
            gravatar_url: String::from("")
        },
    ]
}

#[get("/")]
async fn timeline(flash_messages: IncomingFlashMessages, user: Option<Identity>) -> impl Responder {
    let g_mock = g_mock().unwrap();
    if let Some(user) = user {
    } else {
        Redirect::to("/public").see_other();

    }

    let mut messages = get_messages();
    // you need to login on /register to see any page for now
    for message in &mut messages {
        message.gravatar_url = gravatar_url(&message.gravatar_url);
    }
    
    return TimelineTemplate { 
        messages: messages, 
        request_endpoint: "/", 
        profile_user: Some(User {user_id:Uuid::new_v4(), username:String::from("Name") }), 
        user: Some(g_mock.user ), 
        followed: Some(false),
        flashes: get_flashes(flash_messages),
        title: "Timeline"
    };
}

async fn get_public_timeline() -> Result<()> {
    connect_db().prepare("select message.*, user.* from message, user
    where message.flagged = 0 and message.author_id = user.user_id
    order by message.pub_date desc limit 32")?;

    // let iter = stmt.query_map([], |row| {
    //         Ok(Messages {
    //             text: row.get(0)?,
    //             email: row.get(1)?,
    //             username: row.get(2)?,

    //         })
    //     })?;

    Ok(())
}

#[get("/public")]
async fn public_timeline() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/{username}")]
async fn user_timeline() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/{username}/follow")]
async fn follow_user() -> impl Responder {
    HelloTemplate { name: "AAAA" }
}

#[get("/{username}/unfollow")]
async fn unfollow_user() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[post("/add_message")]
async fn add_message() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/login")]
async fn login(flash_messages: IncomingFlashMessages) -> impl Responder {
    FlashMessage::info("You were logged in!!").send();
    let g_mock = g_mock().unwrap();
    return LoginTemplate {
        user: Some(g_mock.user),
        flashes: get_flashes(flash_messages),
        error: String::from(""),
        username: String::from("a"),
    };
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
}

#[derive(Deserialize)]
struct RegisterInfo {
    username: String,
    email: String,
    password: String,
}

#[post("/login")]
async fn post_login(info: web::Form<LoginInfo>) -> impl Responder {
    let result =connect_db().execute(
        "select * from user where
        username = ?",
        params![info.username],
    ).unwrap();

    if result == 0 {
        FlashMessage::error("Invalid username").send();
        return HelloTemplate {
            name: "RegisterPage",
        };
    }
    HelloTemplate {
        name: "RegisterPage",
    }
}

fn get_flashes(messages: IncomingFlashMessages) -> Vec<String> {
    messages
        .iter()
        .map(|m: &FlashMessage| -> String { m.content().to_string() })
        .collect()
}

#[get("/register")]
async fn register() -> impl Responder {
    let g_mock = g_mock().unwrap();
    RegisterTemplate {
        flashes: vec![],
        error: String::from(""),
        email: String::from(""),
        username: String::from(""),
        password: String::from(""),
        user: Some(g_mock.user),
    }
}

#[post("/register")]
async fn post_register(info: web::Form<RegisterInfo>, request: HttpRequest ) -> impl Responder {

    if info.username.len() == 0 || info.email.len() == 0 || info.password.len() == 0 {
        FlashMessage::error("Missing username email or password").send();
        return Redirect::to("/register").see_other()
    }
    let hash = bcrypt::hash(info.password.clone()).unwrap();

    let result =connect_db().execute(
        "insert into user (
            username, email, pw_hash) values (?, ?, ?)",
        params![info.username, info.email, hash ],
    ).unwrap();
    if result == 0 {
        FlashMessage::error("Invalid info").send();
        return Redirect::to("/register").see_other()
    }
    Identity::login(&request.extensions(), info.username.clone()).unwrap();
    Redirect::to("/").see_other()
}

async fn cookie_test(session: Session) -> impl Responder {
    if let Ok(Some(count)) = session.get::<i32>("counter") {
        let _ = session.insert("counter", count + 1);
    } else {
        let _ = session.insert("counter", 0);
    }

    let count = session.get::<i32>("counter").unwrap().unwrap();
    HttpResponse::Ok().body(format!("Session has been refreshed {count} times"))
}

#[get("/logout")]
async fn logout(session: Session, user: Identity) -> impl Responder {
    FlashMessage::info("You were logged out").send();
    user.logout();
    session.purge();
    Redirect::to("/").see_other()
}
