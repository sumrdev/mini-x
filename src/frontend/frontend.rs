use actix_files as fs;
use actix_identity::config::LogoutBehaviour;
use actix_identity::Identity;
use actix_identity::IdentityMiddleware;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

use actix_web::http::{header, StatusCode};
use actix_web::web::{self, Redirect};

use crate::create_msg;
use crate::create_user;
use crate::establish_connection;
use crate::frontend::flash_messages::*;
use crate::frontend::template_structs::*;
use crate::get_public_messages;
use crate::get_user_by_id;
use crate::get_user_by_name;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::{cookie::Key, get, post, App, HttpResponse, HttpServer, Responder};
use askama_actix::Template;
use chrono::Utc;
use md5::{Digest, Md5};
use pwhash::bcrypt;
use rusqlite::{params, Connection, Result};

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    if !std::fs::metadata(get_database_string()).is_ok() {
        let _ = init_db();
    }
    HttpServer::new(move || {
        App::new()
            .wrap(
                IdentityMiddleware::builder()
                    .logout_behaviour(LogoutBehaviour::DeleteIdentityKeys)
                    .build(),
            )
            .service(fs::Files::new("/static", "./src/frontend/static/").index_file("index.html"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .cookie_http_only(false)
                    .build(),
            )
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
    const SCHEMA_SQL: &str = include_str!("../schema.sql");
    let conn = connect_db();

    conn.execute_batch(&SCHEMA_SQL)?;
    Ok(())
}

fn get_user_id(username: &str) -> i32 {
    let diesel_conn = &mut establish_connection();
    let user = get_user_by_name(diesel_conn, username);
    if let Some(user) = user {
        return user.user_id;
    } else {
        -1
    }

    /* let conn = connect_db();
    let query_result = conn.query_row(
        "SELECT user_id FROM user WHERE username = ?1",
        params![username],
        |row| Ok(row.get(0)),
    );
    query_result.unwrap_or(Ok(-1)).unwrap_or(-1) */

}

fn get_user(user_option: Option<Identity>) -> Option<UserTemplate> {

    if let Some(user) = user_option {
        let diesel_conn = &mut establish_connection();
        let user_id = user.id().unwrap().parse::<i32>().unwrap();
        let user = get_user_by_id(diesel_conn, user_id);
        if let Some(user) = user {
            return Some(UserTemplate {
                user_id: user.user_id,
                username: user.username,
                email: user.email
            });
        }
        
        /* let conn = connect_db();
        let user_id = user.id().unwrap();
        
        match conn.query_row(
            "select * from user where user_id = ?",
            params![user_id],
            |row| {
                Ok(Some(UserTemplate {
                    user_id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                }))
            },
        ) {
            Ok(user) => return user,
            Err(_) => {
                user.logout(); // Call logout if no user is found
                return None;
            }
        } */
    }
    None
}

fn gravatar_url(email: &str) -> String {
    let hash = Md5::digest(email.trim().to_lowercase().as_bytes());

    let hash_str = format!("{:x}", hash);

    format!(
        "https://www.gravatar.com/avatar/{}?d=identicon&s={}",
        hash_str, 48
    )
}

#[get("/")]
async fn timeline(flash: Option<FlashMessages>, user: Option<Identity>) -> impl Responder {
    if let Some(user) = get_user(user) {
        //let mut messages = get_messages();
        // you need to login on /register to see any page for now
        let u = user.user_id;
        let conn = connect_db();
        let prepared_statement = conn.prepare(
            "select message.*, user.* from message, user
        where message.flagged = 0 and message.author_id = user.user_id and (
            user.user_id = ? or
            user.user_id in (select whom_id from follower
                                    where who_id = ?))
        order by message.pub_date desc limit ?",
        );
        let mut stmt = prepared_statement.unwrap();
        let query_result = stmt.query_map([u.clone(), u, 32], |row| {
            Ok(Messages {
                text: row.get(2)?,
                gravatar_url: gravatar_url(&row.get::<_, String>(7)?),
                username: row.get(6)?,
                pub_date: {
                    let date_str: String = row.get(3)?;
                    chrono::DateTime::parse_from_rfc3339(&date_str)
                        .unwrap()
                        .to_utc()
                },
            })
        });

        let messages: Vec<Messages> = query_result.unwrap().map(|m| m.unwrap()).collect();
        println!("{:?}", messages);

        let rendered = TimelineTemplate {
            messages,
            request_endpoint: "timeline",
            profile_user: None,
            user: Some(user),
            followed: Some(false),
            flashes: flash.unwrap_or_default().messages,
            title: String::from("Timeline"),
        }
        .render()
        .unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, "/public"))
            .finish()
    }
}

#[get("/public")]
async fn public_timeline(
    flash_messages: Option<FlashMessages>,
    user: Option<Identity>,
) -> impl Responder {
    let user = get_user(user);
    let conn = connect_db();
    let prepared_statement = conn.prepare(
        "select message.*, user.* from message, user
    where message.flagged = 0 and message.author_id = user.user_id
    order by message.pub_date desc limit 32",
    );
    let mut stmt = prepared_statement.unwrap();
    let query_result = stmt.query_map([], |row| {
        Ok(Messages {
            text: row.get(2)?,
            gravatar_url: gravatar_url(&row.get::<_, String>(7)?),
            username: row.get(6)?,
            pub_date: {
                let date_str: String = row.get(3)?;
                chrono::DateTime::parse_from_rfc3339(&date_str)
                    .unwrap()
                    .to_utc()
            },
        })
    });

    let messages: Vec<Messages> = query_result
        .unwrap()
        .map(|m| {
            println!("{:?}", m);

            m.unwrap()
        })
        .collect();

    TimelineTemplate {
        messages,
        request_endpoint: "/",
        profile_user: None,
        user,
        followed: Some(false),
        flashes: flash_messages.unwrap_or_default().messages,
        title: String::from(""),
    }
}

#[get("/{username}")]
async fn user_timeline(
    path: web::Path<String>,
    user: Option<Identity>,
    flash_messages: Option<FlashMessages>,
) -> impl Responder {
    let username = path.into_inner();
    let conn = connect_db();
    let profile_user = conn.query_row(
        "select * from user where username = ?",
        params![username],
        |row| {
            Ok(UserTemplate {
                user_id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
            })
        },
    );
    if let Ok(profile_user) = profile_user {
        let mut followed = false;
        let user = get_user(user);
        if let Some(user) = user.clone() {
            followed = conn
                .query_row(
                    "select 1 from follower where follower.who_id = ? and follower.whom_id = ?",
                    params![user.user_id, profile_user.user_id],
                    |_| Ok(()),
                )
                .is_ok();
        }
        let conn = connect_db();
        let mut stmt = conn
            .prepare(
                "
            select message.*, user.* from message, user where
            user.user_id = message.author_id and user.user_id = ?
            order by message.pub_date desc limit ?",
            )
            .unwrap();
        let query_result = stmt.query_map([profile_user.user_id, 30], |row| {
            Ok(Messages {
                text: row.get(2)?,
                gravatar_url: gravatar_url(&row.get::<_, String>(7)?),
                username: row.get(6)?,
                pub_date: {
                    let date_str: String = row.get(3)?;
                    chrono::DateTime::parse_from_rfc3339(&date_str)
                        .unwrap()
                        .to_utc()
                },
            })
        });

        let messages: Vec<Messages> = query_result.unwrap().map(|m| m.unwrap()).collect();

        let rendered = TimelineTemplate {
            messages,
            request_endpoint: "user_timeline",
            profile_user: Some(profile_user),
            user,
            followed: Some(followed),
            flashes: flash_messages.unwrap_or_default().messages,
            title: String::from("Timeline"),
        }
        .render()
        .unwrap();
        HttpResponse::Ok().body(rendered)
    } else {
        return HttpResponse::NotFound().finish();
    }
}

#[get("/{username}/follow")]
async fn follow_user(
    user: Option<Identity>,
    path: web::Path<String>,
    _request: HttpRequest,
    session: Session,
) -> impl Responder {
    if let Some(_current_user) = user {
        let _target_username = path.clone();
        let _target_id = get_user_id(&_target_username);
        let _conn = connect_db();
        let sql = "insert into follower (who_id, whom_id) values (?, ?)";
        let _ = _conn.execute(sql, params![_current_user.id().unwrap(), _target_id]);
        let mut message = String::from("You are now following ");
        message.push_str(&_target_username);
        add_flash(session, message.as_str());
    } else {
        return HttpResponse::Found()
            .append_header((header::LOCATION, "User not found"))
            .finish();
    }
    return HttpResponse::Found()
        .append_header((header::LOCATION, format!("/{}", path)))
        .finish();
}

#[get("/{username}/unfollow")]
async fn unfollow_user(
    user: Option<Identity>,
    path: web::Path<String>,
    _request: HttpRequest,
    session: Session,
) -> impl Responder {
    if let Some(_current_user) = user {
        let _target_username = path.clone();
        let _target_id = get_user_id(&_target_username);
        let _conn = connect_db();
        let sql = "delete from follower where who_id=? and whom_id=?";
        let _ = _conn.execute(sql, params![_current_user.id().unwrap(), _target_id]);
        let mut message = String::from("You are no longer following ");
        message.push_str(&_target_username);
        add_flash(session, message.as_str());
    } else {
        return HttpResponse::Found()
            .append_header((header::LOCATION, "User not found"))
            .finish();
    }
    return HttpResponse::Found()
        .append_header((header::LOCATION, format!("/{}", path)))
        .finish();
}

#[post("/add_message")]
async fn add_message(
    user: Option<Identity>,
    msg: web::Form<MessageInfo>,
    session: Session,
) -> impl Responder {
    if let Some(user) = user {
        /* let _ = connect_db().execute(
            "insert into message (author_id, text, pub_date, flagged)
        values (?, ?, ?, 0)",
            params![user.id().unwrap(), msg.text, Utc::now().to_rfc3339()],
        ); */
        let conn = &mut establish_connection();
        let timestamp = Utc::now().to_rfc3339();
        let user_id = user.id().unwrap().parse::<i32>().unwrap();
        let _ = create_msg(conn, &user_id, &msg.text, timestamp, &0);
        add_flash(session, "Your message was recorded");
        return HttpResponse::Found()
            .append_header((header::LOCATION, "/"))
            .finish();
    }
    HttpResponse::Unauthorized()
        .status(StatusCode::UNAUTHORIZED)
        .finish()
}

#[get("/login")]
async fn login(
    flash_messages: Option<FlashMessages>,
    user: Option<Identity>,
    session: Session,
) -> impl Responder {
    if let Some(_) = user {
        add_flash(session, "You are already logged in");
        HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, "/"))
            .finish()
    } else {
        let rendered = LoginTemplate {
            user: None,
            flashes: flash_messages.unwrap_or_default().messages,
            error: String::from(""),
            username: String::from(""),
        }
        .render()
        .unwrap();
        HttpResponse::Ok().body(rendered)
    }
}

#[post("/login")]
async fn post_login(
    info: web::Form<LoginInfo>,
    request: HttpRequest,
    session: Session,
) -> impl Responder {
    let result: Result<String> = connect_db().query_row(
        "select pw_hash from user where username = ?",
        params![info.username],
        |row| row.get(0),
    );
    if result.is_err() {
        add_flash(session, "Invalid username");
        return HttpResponse::Found()
            .append_header((header::LOCATION, "/login"))
            .finish();
    }
    println!("{:?}", result);
    if let Ok(stored_hash) = result {
        if bcrypt::verify(info.password.clone(), &stored_hash) {
            // Successful login
            let user_id = get_user_id(&info.username);
            let _ = Identity::login(&request.extensions(), user_id.to_string());
            add_flash(session, "You were logged in");

            return HttpResponse::Found()
                .append_header((header::LOCATION, "/"))
                .finish();
        }
    }

    // Password incorrect
    add_flash(session, "Invalid password");
    return HttpResponse::Found()
        .append_header((header::LOCATION, "/login"))
        .finish();
}

#[get("/register")]
async fn register(flash_messages: Option<FlashMessages>) -> impl Responder {
    RegisterTemplate {
        flashes: flash_messages.unwrap_or_default().messages,
        error: String::from(""),
        email: String::from(""),
        username: String::from(""),
        password: String::from(""),
        user: None,
    }
}

#[post("/register")]
async fn post_register(info: web::Form<RegisterInfo>, session: Session) -> impl Responder {
    if info.username.len() == 0 {
        add_flash(session, "You have to enter a username");
        return Redirect::to("/register").see_other();
    } else if info.email.len() == 0 || !info.email.contains("@") {
        add_flash(session, "You have to enter a valid email address");
        return Redirect::to("/register").see_other();
    } else if info.password.len() == 0 {
        add_flash(session, "You have to enter a password");
        return Redirect::to("/register").see_other();
    } else if info.password != info.password2 {
        add_flash(session, "The two passwords do not match");
        return Redirect::to("/register").see_other();
    } else if get_user_id(&info.username) != -1 {
        add_flash(session, "The username is already taken");
        return Redirect::to("/register").see_other();
    }

    let hash = bcrypt::hash(info.password.clone()).unwrap();

    let conn = &mut establish_connection();
    let _ = create_user(conn, &info.username, &info.email, &hash);

    add_flash(
        session,
        "You were successfully registered and can login now",
    );
    Redirect::to("/login").see_other()
}
#[get("/logout")]
async fn logout(user: Identity, session: Session) -> impl Responder {
    add_flash(session, "You were logged out");
    user.logout();
    Redirect::to("/public").see_other()
}
