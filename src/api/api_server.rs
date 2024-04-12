use crate::api::api_structs::*;
use crate::{
    create_msg, create_user, establish_connection, follow, get_followers, get_public_messages,
    get_timeline, get_user_by_name, set_latest, unfollow,
};
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use chrono::Utc;
use diesel::PgConnection;
use pwhash::bcrypt;
use std::collections::HashMap;

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
            .wrap(prometheus.clone())
            .wrap(Logger::default())
            .service(get_latest)
            .service(post_register)
            .service(messages_per_user_get)
            .service(messages_per_user_post)
            .service(messages_api)
            .service(follows_get)
            .service(follows_post)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}

fn get_user_id(username: &str) -> Option<i32> {
    let conn = &mut establish_connection();
    get_user_by_name(conn, username).map(|user| user.user_id)
}

fn update_latest(conn: &mut PgConnection, query: web::Query<Latest>) {
    set_latest(conn, query.latest);
}

#[get("/latest")]
async fn get_latest() -> impl Responder {
    let conn = &mut establish_connection();
    let latest = crate::get_latest(conn);
    HttpResponse::Ok().json(Latest { latest })
}

#[post("/register")]
async fn post_register(info: web::Json<RegisterInfo>, query: web::Query<Latest>) -> impl Responder {
    let conn = &mut establish_connection();
    update_latest(conn, query);

    let user_exists = get_user_id(&info.username);

    let error = if info.username.is_empty() {
        Some(String::from("You have to enter a username"))
    } else if info.email.is_empty() {
        Some(String::from("You have to enter a valid email address"))
    } else if info.pwd.is_empty() {
        Some(String::from("You have to enter a password"))
    } else if user_exists.is_some() {
        Some(String::from("The username is already taken"))
    } else {
        None
    };

    if let Some(err_msg) = error {
        let reg_err = RegisterError {
            status: 400,
            error_msg: err_msg.to_string(),
        };
        HttpResponse::BadRequest().json(reg_err)
    } else {
        let hash = bcrypt::hash(info.pwd.clone()).unwrap();

        let _ = create_user(conn, &info.username, &info.email, &hash);
        HttpResponse::NoContent().json(String::from(""))
    }
}

#[get("/msgs")]
async fn messages_api(
    amount: web::Query<MessageAmount>,
    query: web::Query<Latest>,
) -> impl Responder {
    let conn = &mut establish_connection();
    update_latest(conn, query);
    let messages: Vec<Message> = get_public_messages(conn, amount.no)
        .into_iter()
        .map(|(msg, user)| Message {
            content: msg.text,
            user: user.username,
            pub_date: chrono::DateTime::parse_from_rfc3339(&msg.pub_date)
                .unwrap()
                .to_utc(),
        })
        .collect();

    HttpResponse::Ok().json(messages)
}

#[get("msgs/{username}")]
async fn messages_per_user_get(
    path: web::Path<String>,
    amount: web::Query<MessageAmount>,
    query: web::Query<Latest>,
) -> impl Responder {
    let conn = &mut establish_connection();
    update_latest(conn, query);
    let username = &path;
    if let Some(user_id) = get_user_id(username) {
        let messages: Vec<Message> = get_timeline(conn, user_id, amount.no)
            .into_iter()
            .map(|(msg, user)| Message {
                content: msg.text,
                user: user.username,
                pub_date: chrono::DateTime::parse_from_rfc3339(&msg.pub_date)
                    .unwrap()
                    .to_utc(),
            })
            .collect();

        HttpResponse::Ok().json(messages)
    } else {
        HttpResponse::NotFound().json("")
    }
}

#[post("msgs/{username}")]
async fn messages_per_user_post(
    path: web::Path<String>,
    msg: web::Json<MessageContent>,
    query: web::Query<Latest>,
) -> impl Responder {
    let conn = &mut establish_connection();
    update_latest(conn, query);
    let username = &path;
    if let Some(user_id) = get_user_id(username) {
        let _ = create_msg(conn, &user_id, &msg.content, Utc::now().to_rfc3339(), &0);
        HttpResponse::NoContent().json("")
    } else {
        HttpResponse::NotFound().json("")
    }
}

#[get("fllws/{username}")]
async fn follows_get(
    path: web::Path<String>,
    amount: web::Query<MessageAmount>,
    query: web::Query<Latest>,
) -> impl Responder {
    let conn = &mut establish_connection();
    update_latest(conn, query);
    let username = &path;
    if let Some(user_id) = get_user_id(username) {
        let followers = get_followers(conn, user_id, amount.no);
        let followers = followers.into_iter().map(|user| user.username).collect();

        HttpResponse::Ok().json(Follows { follows: followers })
    } else {
        HttpResponse::NotFound().json("")
    }
}

#[post("fllws/{username}")]
async fn follows_post(
    path: web::Path<String>,
    follow_param: web::Json<FollowParam>,
    query: web::Query<Latest>,
) -> impl Responder {
    let conn = &mut establish_connection();
    update_latest(conn, query);
    let username = &path;
    if let Some(user_id) = get_user_id(username) {
        let follow_param = follow_param.into_inner();
        if let Some(follow_username) = follow_param.follow {
            if let Some(follow_user_id) = get_user_id(&follow_username) {
                follow(conn, user_id, follow_user_id);
                return HttpResponse::NoContent();
            }
        } else if let Some(unfollow_username) = follow_param.unfollow {
            if let Some(unfollow_user_id) = get_user_id(&unfollow_username) {
                let conn = &mut establish_connection();
                unfollow(conn, user_id, unfollow_user_id);
                return HttpResponse::NoContent();
            }
        }

        HttpResponse::BadRequest()
    } else {
        HttpResponse::NotFound()
    }
}
