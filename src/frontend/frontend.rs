use std::collections::HashMap;

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
use crate::follow;
use crate::frontend::flash_messages::*;
use crate::frontend::template_structs::*;
use crate::get_passwd_hash;
use crate::get_public_messages;
use crate::get_timeline;
use crate::get_user_by_id;
use crate::get_user_by_name;
use crate::unfollow;
use crate::get_user_timeline;
use crate::is_following;
use crate::Messages;
use crate::Users;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::{cookie::Key, get, post, App, HttpResponse, HttpServer, Responder};
use askama_actix::Template;
use chrono::Utc;
use md5::{Digest, Md5};
use pwhash::bcrypt;
use prometheus::Opts;
use actix_web_prom::{PrometheusMetricsBuilder};

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

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
            .wrap(prometheus.clone())
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

fn get_user_id(username: &str) -> i64 {
    let diesel_conn = &mut establish_connection();
    let user = get_user_by_name(diesel_conn, username);
    if let Some(user) = user {
        user.user_id
    } else {
        -1
    }
}

fn get_user_template_by_name(username: &str) -> Option<UserTemplate> {
    let diesel_conn = &mut establish_connection();
    let user = get_user_by_name(diesel_conn, username);
    if let Some(user) = user {
        Some(UserTemplate {
            user_id: user.user_id,
            username: user.username,
            email: user.email
        })
    } else {
        None
    }
}

fn get_user_template(user_id: i64) -> Option<UserTemplate> {
    let diesel_conn = &mut establish_connection();
    let user = get_user_by_id(diesel_conn, user_id);
    if let Some(user) = user {
        Some(UserTemplate {
            user_id: user.user_id,
            username: user.username,
            email: user.email
        })
    } else {
        None
    }
}

fn get_user(user_option: Option<Identity>) -> Option<UserTemplate> {
    if let Some(user) = user_option {
        let user_id = user.id().unwrap().parse::<i64>().unwrap();
        get_user_template(user_id)
    } else {
        None
    }
}

fn gravatar_url(email: &str) -> String {
    let hash = Md5::digest(email.trim().to_lowercase().as_bytes());

    let hash_str = format!("{:x}", hash);

    format!(
        "https://www.gravatar.com/avatar/{}?d=identicon&s={}",
        hash_str, 48
    )
}

fn format_messages (messages: Vec<(Messages, Users)>) -> Vec<MessageTemplate>{
    let mut messages_for_template: Vec<MessageTemplate> = Vec::new();
    for (msg, user) in messages {
        let message = MessageTemplate {
            text: msg.text,
            username: user.username,
            gravatar_url: gravatar_url(&user.email),
            pub_date: chrono::DateTime::parse_from_rfc3339(&msg.pub_date).unwrap().to_utc()
        };
        messages_for_template.push(message)
    }
    messages_for_template
}

#[get("/")]
async fn timeline(flash: Option<FlashMessages>, user: Option<Identity>) -> impl Responder {
    if let Some(user) = get_user(user) {
        let diesel_conn = &mut establish_connection();
        let messages = format_messages(get_timeline(diesel_conn,  user.user_id, 32));

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
    let diesel_conn = &mut establish_connection();
    let messages = get_public_messages(diesel_conn, 32);
    let messages_for_template = format_messages(messages);

    TimelineTemplate {
        messages: messages_for_template,
        request_endpoint: "/",
        profile_user: None,
        user,
        followed: Some(false),
        flashes: flash_messages.unwrap_or_default().messages,
        title: String::from(""),
    }
}

#[get("/{username}")]
async fn user_timeline(path: web::Path<String>, user: Option<Identity>, flash_messages: Option<FlashMessages>) -> impl Responder {
    let username = path.into_inner();
    let profile_user = get_user_template_by_name(&username);
    if let Some(profile_user) = profile_user{
        let mut followed = false;
        let user = get_user(user);
        let conn = &mut establish_connection();
        if let Some(user) = user.clone() {
            followed = is_following(conn, profile_user.user_id, user.user_id)
        }
        let messages = format_messages(get_user_timeline(conn, profile_user.user_id, 30));
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
        let conn = &mut establish_connection();
        let _ = follow(conn, _current_user.id().unwrap().parse::<i64>().unwrap(),_target_id);
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
        let conn = &mut establish_connection();
        let _ = unfollow(conn, _current_user.id().unwrap().parse::<i64>().unwrap(), _target_id);
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
        let conn = &mut establish_connection();
        let timestamp = Utc::now().to_rfc3339();
        let user_id = user.id().unwrap().parse::<i64>().unwrap();
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
        add_flash(session, "You are already loggei64n");
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
    let conn = &mut establish_connection();
    let result = get_passwd_hash(conn, &info.username);
    if result.is_none() {
        add_flash(session, "Invalid username");
        return HttpResponse::Found()
            .append_header((header::LOCATION, "/login"))
            .finish();
    }
    println!("{:?}", result);
    if let Some(stored_hash) = result {
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
