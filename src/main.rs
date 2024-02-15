
use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::http::{self, header, Method, StatusCode};
use actix_web::web::{self, method, Redirect};
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
use pwhash::bcrypt;

#[derive(Template)] // this will generate the code...
#[template(path = "../templates/hello.html")] // using the template in this path, relative
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[derive(Clone)]
struct User {
    user_id: i32,
    username: String,
    email: String
}
struct G {
    db: Connection,
    user: User,
}
#[derive(Debug)]
struct Messages {
    text: String,
    username: String,
    pub_date: DateTime<Utc>,
    gravatar_url: String
}

#[derive(Template)]
#[template(path = "../templates/timeline.html")]
struct TimelineTemplate<'a> {
    messages: Vec<Messages>, 
    user: Option<User>,
    request_endpoint: &'a str, 
    profile_user: Option<User>,
    followed: Option<bool>, 
    flashes: Vec<String>,
    title: String
}

#[derive(Template)]
#[template(path = "../templates/login.html")]
struct LoginTemplate {
    user: Option<User>,
    //g: Option<G>,
    error: String,
    flashes: Vec<String>,
    username: String, 
}

#[derive(Template)]
#[template(path = "../templates/register.html")]
struct RegisterTemplate {
    user: Option<User>,
    email: String,
    username: String,
    password: String,
    flashes: Vec<String>,
    error: String,
}

#[derive(Deserialize)]
struct MessageInfo {
    text: String,
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
    .bind(("0.0.0.0", 5000))?
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

fn get_user_id(username: &str) -> i32 {
    let conn = connect_db();
    let query_result = conn.query_row("SELECT user_id FROM user WHERE username = ?1", params![username], |row| { Ok(row.get(0))});
    query_result.unwrap().unwrap()
}

fn get_user(user_option: Option<Identity>) -> Option<User> {
    if let Some(user) = user_option {
        let conn = connect_db();
        let user_id = user.id().unwrap();
        
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

fn gravatar_url(email: &str) -> String {
    let hash = Md5::digest(email.trim().to_lowercase().as_bytes());
    
    let hash_str = format!("{:x}", hash);
    
    format!("https://www.gravatar.com/avatar/{}?d=identicon&s={}", hash_str, 48)
}

fn get_flashes(messages: IncomingFlashMessages) -> Vec<String> {
    messages
        .iter()
        .map(|m: &FlashMessage| -> String { m.content().to_string() })
        .collect()
}


#[get("/")]
async fn timeline(flash_messages: IncomingFlashMessages, user: Option<Identity>) -> impl Responder {
    if let Some(user) = get_user(user) {
        //let mut messages = get_messages();
        // you need to login on /register to see any page for now

        let u = user.user_id;
        let conn = connect_db();
        let prepared_statement = conn.prepare("select message.*, user.* from message, user
        where message.flagged = 0 and message.author_id = user.user_id and (
            user.user_id = ? or
            user.user_id in (select whom_id from follower
                                    where who_id = ?))
        order by message.pub_date desc limit ?");
        let mut stmt = prepared_statement.unwrap();
        let query_result = stmt.query_map([u.clone(),u, 32], |row| {
            Ok(Messages {
                text: row.get(2)?,
                gravatar_url: gravatar_url(&row.get::<_, String>(7)?),
                username: row.get(6)?,
                pub_date: {
                    let date_str: String = row.get(3)?;
                    chrono::DateTime::parse_from_rfc3339(&date_str).unwrap().to_utc()
                }
            })
        });

        let messages : Vec<Messages> = query_result.unwrap()
        .map(|m| { m.unwrap()})
        .collect();
        println!("{:?}",messages);

        
        let rendered = TimelineTemplate { 
            messages, 
            request_endpoint: "timeline", 
            profile_user: None, 
            user: Some(user),
            followed: Some(false),
            flashes: get_flashes(flash_messages),
            title: String::from("Timeline")
        }.render().unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, "/public"))
        .finish()

    }

}

#[get("/public")]
async fn public_timeline(flash_messages: IncomingFlashMessages, user: Option<Identity>) -> impl Responder {
    let user = get_user(user);
    let conn = connect_db();
    let prepared_statement = conn.prepare("select message.*, user.* from message, user
    where message.flagged = 0 and message.author_id = user.user_id
    order by message.pub_date desc limit 32");
    let mut stmt = prepared_statement.unwrap();
    let query_result = stmt.query_map([], |row| {
        Ok(Messages {
            text: row.get(2)?,
            gravatar_url: gravatar_url(&row.get::<_, String>(7)?),
            username: row.get(6)?,
            pub_date: {
                let date_str: String = row.get(3)?;
                chrono::DateTime::parse_from_rfc3339(&date_str).unwrap().to_utc()
            }
        })
    });

    let messages : Vec<Messages> = query_result.unwrap()
    .map(|m| { 
        println!("{:?}",m);

        m.unwrap()
    })
    .collect();

    TimelineTemplate { 
        messages, 
        request_endpoint: "/", 
        profile_user: None,
        user, 
        followed: Some(false),
        flashes: get_flashes(flash_messages),
        title: String::from("")
    }
}

#[get("/{username}")]
async fn user_timeline(path: web::Path<(String,)>, user: Option<Identity>, flash_messages: IncomingFlashMessages) -> impl Responder {
    let username = &path.0;
    let conn = connect_db();
    let profile_user = conn.query_row("select * from user where username = ?", params![username], |row| {
        Ok(User {
            user_id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
        })
    });
    if let Ok(profile_user) = profile_user {
        let mut followed = false;
        let user = get_user(user);
        if let Some(user) = user.clone() {
            followed = conn.query_row(
                "select 1 from follower where follower.who_id = ? and follower.whom_id = ?", 
                params![user.user_id, profile_user.user_id], 
                |_| {Ok(())}
            ).is_ok();
        }
        let conn = connect_db();
        let mut stmt = conn.prepare("
            select message.*, user.* from message, user where
            user.user_id = message.author_id and user.user_id = ?
            order by message.pub_date desc limit ?").unwrap();
        let query_result = stmt.query_map([profile_user.user_id, 30], |row| {
            Ok(Messages {
                text: row.get(2)?,
                gravatar_url: gravatar_url(&row.get::<_, String>(7)?),
                username: row.get(6)?,
                pub_date: {
                    let date_str: String = row.get(3)?;
                    chrono::DateTime::parse_from_rfc3339(&date_str).unwrap().to_utc()
                }
            })
        });

        let messages : Vec<Messages> = query_result.unwrap()
        .map(|m| { m.unwrap()})
        .collect();

        
        let rendered = TimelineTemplate { 
            messages, 
            request_endpoint: "timeline", 
            profile_user: Some(profile_user),
            user,
            followed: Some(followed),
            flashes: get_flashes(flash_messages),
            title: String::from("Timeline")
        }.render().unwrap();
        HttpResponse::Ok().body(rendered)
    }
    else {
        return HttpResponse::NotFound().finish();
    }
}

#[get("/{username}/follow")]
async fn follow_user(user: Option<Identity>, path: web::Path<String>, _request: HttpRequest) -> impl Responder {
    if let Some(_current_user) = user {
        let _target_username = path.clone();
        let _target_id = get_user_id(&_target_username);
        let _conn = connect_db();
        let sql = "insert into follower (who_id, whom_id) values (?, ?)";
        let _ = _conn.execute(sql, params![_current_user.id().unwrap(), _target_id]);
        
    } else {
        return HttpResponse::Found()
    .append_header((header::LOCATION, "User not found")).finish()
    }
    return HttpResponse::Found()
    .append_header((header::LOCATION, format!("/{}", path))).finish()
}

#[get("/{username}/unfollow")]
async fn unfollow_user(user: Option<Identity>, path: web::Path<String>, _request: HttpRequest) -> impl Responder {
    if let Some(_current_user) = user {
        let _target_username = path.clone();
        let _target_id = get_user_id(&_target_username);
        let _conn = connect_db();
        let sql = "delete from follower where who_id=? and whom_id=?";
        let _ = _conn.execute(sql, params![_current_user.id().unwrap(), _target_id]);
        
    } else {
        return HttpResponse::Found()
    .append_header((header::LOCATION, "User not found")).finish()
    }
    return HttpResponse::Found()
    .append_header((header::LOCATION, format!("/{}", path))).finish()
}

#[post("/add_message")]
async fn add_message(user: Option<Identity>, msg: web::Form<MessageInfo>) -> impl Responder {
    println!("{}", "a");
    if let Some(user) = user {
        let _ = connect_db().execute("insert into message (author_id, text, pub_date, flagged)
        values (?, ?, ?, 0)", params![user.id().unwrap(),msg.text, Utc::now().to_rfc3339()]);
        FlashMessage::info("Your message was recorded").send();
        return HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
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

#[post("/login")]
async fn post_login(info: web::Form<LoginInfo>, request: HttpRequest) -> impl Responder {
    let result : Result<String> = connect_db()
        .query_row(
            "select pw_hash from user where username = ?",
            params![info.username],
            |row| row.get(0)
        );
    println!("{:?}", result);
    if let Ok(stored_hash) = result {
        if bcrypt::verify(info.password.clone(), &stored_hash) {
            // Successful login
            let user_id = get_user_id(&info.username);
            let _ = Identity::login(&request.extensions(), user_id.to_string());
            FlashMessage::info("You were logged in").send();
            return Redirect::to("/").see_other()
        }
    }

    // Password incorrect
    FlashMessage::error("Invalid username or password").send();
    return Redirect::to("/login").see_other();
}

#[get("/register")]
async fn register() -> impl Responder {
    RegisterTemplate {
        flashes: vec![],
        error: String::from(""),
        email: String::from(""),
        username: String::from(""),
        password: String::from(""),
        user: None
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
    FlashMessage::info("You were successfully registered and can login now").send();
    Redirect::to("/login").see_other()
}

#[get("/logout")]
async fn logout(user: Identity) -> impl Responder {
    FlashMessage::info("You were logged out").send();
    user.logout();
    Redirect::to("/public").see_other()
}
