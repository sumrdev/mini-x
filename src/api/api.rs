use crate::api::api_structs::*;
use crate::{create_msg, create_user, establish_connection, follow, get_followers, get_public_messages, get_timeline, get_user_by_name, unfollow};
use actix_files as fs;
use actix_web::web;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;
use chrono::{DateTime, Local, Utc};
use log::LevelFilter;
use pwhash::bcrypt;
use std::collections::HashMap;
use std::sync::Mutex;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let latest = web::Data::new(LatestAction {
        latest: Mutex::new(-1),
    });
    let local: DateTime<Local> = Local::now();

    // Format the date as a string in the desired format
    let date = local.format("%m_%e_%y-%H:%M:%S").to_string();

    let _ = simple_logging::log_to_file(format!("{}.log", date), LevelFilter::Warn);

    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(latest.clone())
            .wrap(prometheus.clone())
            .service(fs::Files::new("/static", "./static/").index_file("index.html"))
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
    get_user_by_name(conn, username).and_then(|user| Some(user.user_id))
}

fn update_latest(query: web::Query<Latest>, latest_action: web::Data<LatestAction>) {
    let mut latest = latest_action.latest.lock().unwrap();
    *latest = query.latest;
}

#[get("/latest")]
async fn get_latest(latest_action: web::Data<LatestAction>) -> impl Responder {
    let latest = latest_action.latest.lock().unwrap();
    HttpResponse::Ok().json(Latest { latest: *latest })
}

#[post("/register")]
async fn post_register(
    info: web::Json<RegisterInfo>,
    query: web::Query<Latest>,
    latest: web::Data<LatestAction>,
) -> impl Responder {
    update_latest(query, latest);

    let user_exists = get_user_id(&info.username);

    let error = 
    if info.username.len() == 0 {
        Some(String::from("You have to enter a username"))
    } else if info.email.len() == 0 {
        Some(String::from("You have to enter a valid email address"))
    } else if info.pwd.len() == 0 {
        Some(String::from("You have to enter a password"))
    } else if let Some(_) = user_exists {
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

        let conn = &mut establish_connection();
        let _ = create_user(conn, &info.username, &info.email, &hash);
        HttpResponse::NoContent().json(String::from(""))
    }
}

#[get("/msgs")]
async fn messages_api(
    amount: web::Query<MessageAmount>,
    query: web::Query<Latest>,
    latest_action: web::Data<LatestAction>,
) -> impl Responder {
    update_latest(query, latest_action);
    let conn = &mut establish_connection();
    let messages: Vec<Message> = get_public_messages(conn, amount.no)
        .into_iter()
        .map(|(msg, user)| Message {
            content: msg.text,
            user: user.username,
            pub_date: chrono::DateTime::parse_from_rfc3339(&msg.pub_date).unwrap().to_utc()
        }).collect();

    HttpResponse::Ok().json(messages)
}

#[get("msgs/{username}")]
async fn messages_per_user_get(
    path: web::Path<(String,)>,
    amount: web::Query<MessageAmount>,
    query: web::Query<Latest>,
    latest_action: web::Data<LatestAction>,
) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let conn = &mut establish_connection();
        let messages: Vec<Message> = get_timeline(conn, user_id, amount.no)
            .into_iter()
            .map(|(msg, user)| Message {
                content: msg.text,
                user: user.username,
                pub_date: chrono::DateTime::parse_from_rfc3339(&msg.pub_date).unwrap().to_utc()
            }).collect();
        

        HttpResponse::Ok().json(messages)
    } else {
        HttpResponse::NotFound().json("")
    }
}

#[post("msgs/{username}")]
async fn messages_per_user_post(
    path: web::Path<(String,)>,
    msg: web::Json<MessageContent>,
    query: web::Query<Latest>,
    latest_action: web::Data<LatestAction>,
) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let conn = &mut establish_connection();
        let _ = create_msg(conn, &user_id, &msg.content, Utc::now().to_rfc3339(), &0);
        HttpResponse::NoContent().json("")
    } else {
        HttpResponse::NotFound().json("")
    }
}

#[get("fllws/{username}")]
async fn follows_get(
    path: web::Path<(String,)>,
    amount: web::Query<MessageAmount>,
    query: web::Query<Latest>,
    latest_action: web::Data<LatestAction>,
) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let conn = &mut establish_connection();
        let followers = get_followers(conn, user_id, amount.no);
        let followers = followers
            .into_iter()
            .map(|user| user.username)
            .collect();

        HttpResponse::Ok().json(Follows { follows: followers })
    } else {
        HttpResponse::NotFound().json("")
    }
}

#[post("fllws/{username}")]
async fn follows_post(
    path: web::Path<(String,)>,
    follow_param: web::Json<FollowParam>,
    query: web::Query<Latest>,
    latest_action: web::Data<LatestAction>,
) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let follow_param = follow_param.into_inner();
        if let Some(follow_username) = follow_param.follow {
            if let Some(follow_user_id) = get_user_id(&follow_username) {
                let conn = &mut establish_connection();
                let _ = follow(conn, user_id, follow_user_id);
                return HttpResponse::NoContent();
            }
        } else if let Some(unfollow_username) = follow_param.unfollow {
            if let Some(unfollow_user_id) = get_user_id(&unfollow_username) {
                let conn = &mut establish_connection();
                let _ = unfollow(conn, user_id, unfollow_user_id);
                return HttpResponse::NoContent();
            }
        }

        HttpResponse::BadRequest()
    } else {
        HttpResponse::NotFound()
    }
}
