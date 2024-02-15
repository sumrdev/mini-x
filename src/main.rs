use std::future::IntoFuture;

use actix_files as fs;
use actix_identity::error::GetIdentityError;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::http::{header, StatusCode};
use actix_web::rt::System;
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
    user_id: i32,
    username: String,
    email: String
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
            .service(public_timeline)
            .service(login)
            .service(post_login)
            .service(logout)
            .service(user_timeline)
            .service(follow_user)
            .service(unfollow_user)
            .service(add_message)
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

fn g(user: Identity) -> Option<User> {
    let conn = connect_db();
    if let Ok(user_id) = user.id() {

        
        let user = conn.query_row("select * from user where user_id = ?", params![user_id], |row| {
            Ok(Some(User {
                user_id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            }))
        }).unwrap();
        return user;
    }
    None
}

fn g_mock() -> Result<G> {
    let connection = Connection::open_in_memory()?;

    Ok(G {
        db: connection,
        user: User {
            user_id: 5,
            username: String::from("Test Name"),
            email: String::from("email@mail.com")
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
        let mut messages = get_messages();
        // you need to login on /register to see any page for now
        for message in &mut messages {
            message.gravatar_url = gravatar_url(&message.gravatar_url);
        }
        println!("{:?}",user.id());
        
        return TimelineTemplate { 
            messages: messages, 
            request_endpoint: "/", 
            profile_user: Some(User {user_id: 0, username:String::from("Name"), email: String::from("mail2") }), 
            user: Some(g_mock.user ), 
            followed: Some(false),
            flashes: get_flashes(flash_messages),
            title: "Hello USER?"
        }
    } else {
        Redirect::to("/public").see_other();
        let mut messages = get_messages();
        return TimelineTemplate { 
            messages: messages, 
            request_endpoint: "/", 
            profile_user: Some(User {user_id:1, username:String::from("Name"), email:String::from("email")}), 
            user: Some(g_mock.user ), 
            followed: Some(false),
            flashes: get_flashes(flash_messages),
            title: ""
        }
    }

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
async fn public_timeline(flash_messages: IncomingFlashMessages) -> impl Responder {
    let g_mock = g_mock().unwrap();
    let mut messages = get_messages();
    return TimelineTemplate { 
        messages: messages, 
        request_endpoint: "/", 
        profile_user: Some(User {user_id:1, username:String::from("Name"), email:String::from("email")}),
        user: Some(g_mock.user ), 
        followed: Some(false),
        flashes: get_flashes(flash_messages),
        title: ""
    }
}

#[get("/{username}")]
async fn user_timeline(path: web::Path<(String,)>) -> impl Responder {
    let username = &path.0;
    println!("{}", username.clone());
    return HelloTemplate { name: "aaa" };
}

#[get("/{username}/follow")]
async fn follow_user() -> impl Responder {
    HelloTemplate { name: "AAAA" }
}

#[get("/{username}/unfollow")]
async fn unfollow_user() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[derive(Deserialize)]
struct MessageInfo {
    text: String,
}

#[post("/add_message")]
async fn add_message(user: Option<Identity>, msg: web::Form<MessageInfo>,) -> impl Responder {
    if let Some(user) = user {
        let _ = connect_db().execute("insert into message (author_id, text, pub_date, flagged)
        values (?, ?, ?, 0)", params![user.id().unwrap(),msg.text, Utc::now().to_rfc3339()]);
        return HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/"))
        .finish();
    }
    HttpResponse::Unauthorized()
     .status(StatusCode::UNAUTHORIZED) 
     .finish()
}

#[get("/login")]
async fn login(flash_messages: IncomingFlashMessages, user: Option<Identity>) -> impl Responder {
    if let Some(_) = user {
        FlashMessage::info("You are already logged in").send();
        HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/"))
        .finish()
    }
    else {
        let rendered = LoginTemplate {
            user: None,
            flashes: get_flashes(flash_messages),
            error: String::from(""),
            username: String::from(""),
        }.render().unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterInfo {
    username: String,
    email: String,
    password: String,
}

#[post("/login")]
async fn post_login(info: web::Form<LoginInfo>, request: HttpRequest) -> impl Responder {
    let result : Result<String> = connect_db()
        .query_row(
            "select pw_hash from user where username = ?",
            params![info.username],
            |row| row.get(0)
        );
    if let Ok(stored_hash) = result {
        if bcrypt::verify(info.password.clone(), &stored_hash) {
            // Successful login
            Identity::login(&request.extensions(), info.username.clone());
            return Redirect::to("/").see_other()
        }
    }

    // Password incorrect
    FlashMessage::error("Invalid username or password").send();
    return Redirect::to("/login").see_other();
    
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
async fn logout(user: Identity) -> impl Responder {
    FlashMessage::info("You were logged out").send();
    user.logout();
    Redirect::to("/public").see_other()
}
