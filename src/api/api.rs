
use std::sync::Mutex;
use actix_files as fs;
use actix_web::web;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::{DateTime, Local, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use pwhash::bcrypt;
use crate::api::api_structs::*;
use crate::{create_user, establish_connection};
use log::LevelFilter;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    let latest = web::Data::new(LatestAction {latest: Mutex::new(-1)});
    if !std::fs::metadata(get_database_string()).is_ok() {
        let _ = init_db();
    }
    let local: DateTime<Local> = Local::now();

    // Format the date as a string in the desired format
    let date = local.format("%m_%e_%y-%H:%M:%S").to_string();

    let _ = simple_logging::log_to_file(format!("{}.log", date), LevelFilter::Warn);
    HttpServer::new(move || {
        App::new()
            .app_data(latest.clone())
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

fn get_database_string() -> String {
    String::from("/databases/mini-x.db")
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

fn get_user_id(username: &str) -> Option<i32> {
    let conn = connect_db();
    let res = conn.query_row("SELECT user_id FROM user WHERE username = ?1", params![username], |row| { row.get(0)}).optional().unwrap();
    res
}

fn update_latest(query: web::Query<Latest>, latest_action: web::Data<LatestAction>){
    let mut latest = latest_action.latest.lock().unwrap();
    *latest = query.latest;
}

#[get("/latest")]
async fn get_latest(latest_action: web::Data<LatestAction>) -> impl Responder {
    let latest = latest_action.latest.lock().unwrap();
    HttpResponse::Ok().json(Latest{latest: *latest})
}

#[post("/register")]
async fn post_register(info: web::Json<RegisterInfo>, query: web::Query<Latest>, latest: web::Data<LatestAction>) -> impl Responder {

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

    let hash = bcrypt::hash(info.pwd.clone()).unwrap();

    let conn = &mut establish_connection();
    let _ = create_user(conn, &info.username, &info.email, &hash);

    /* let _ = connect_db().execute(
        "insert into user (
            username, email, pw_hash) values (?, ?, ?)",
        params![info.username, info.email, hash ],
    ); */

    if let Some(err_msg) = error {
        let reg_err = RegisterError {status: 400, error_msg: err_msg.to_string()};
        HttpResponse::BadRequest().json(reg_err)
    }
    else {
        HttpResponse::NoContent().json(String::from(""))
    }
    
}

#[get("/msgs")]
async fn messages_api(amount: web::Query<MessageAmount>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder{
    update_latest(query, latest_action);

    let conn = connect_db();
    let mut stmt = conn.prepare("
        SELECT message.*, user.* FROM message, user
        WHERE message.flagged = 0 AND message.author_id = user.user_id
        ORDER BY message.pub_date DESC LIMIT ?").unwrap();
    let result = stmt.query_map([amount.no], |row| { 
        Ok(Message {
            content: row.get(2)?,
            user: row.get(6)?,
            pub_date: {
                let date_str: String = row.get(3)?;
                chrono::DateTime::parse_from_rfc3339(&date_str).unwrap().to_utc()
            }
        }) 
    });
    
    let messages: Vec<Message> = result.unwrap().skip_while(|m| m.is_err()).map(|m| m.unwrap()).collect();
    
    HttpResponse::Ok().json(messages)
}

#[get("msgs/{username}")]
async fn messages_per_user_get(path: web::Path<(String,)>, amount: web::Query<MessageAmount>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let conn = connect_db();
        let mut stmt = conn.prepare("
            SELECT message.*, user.* FROM message, user
            WHERE message.flagged = 0 AND
            user.user_id = message.author_id AND user.user_id = ?
            ORDER BY message.pub_date DESC LIMIT ?").unwrap();
        let result = stmt.query_map([user_id, amount.no], |row| { 
            Ok(Message {
                content: row.get(2)?,
                user: row.get(6)?,
                pub_date: {
                    let date_str: String = row.get(3)?;
                    chrono::DateTime::parse_from_rfc3339(&date_str).unwrap().to_utc()
                }
            }) 
        });
        
        let messages: Vec<Message> = result.unwrap().skip_while(|m| m.is_err()).map(|m| m.unwrap()).collect();
    
        HttpResponse::Ok().json(messages)
    }
    else {
        HttpResponse::NotFound().json("")
    }
}

#[post("msgs/{username}")]
async fn messages_per_user_post(path: web::Path<(String,)>, msg: web::Json<MessageContent>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let stmt = "insert into message (author_id, text, pub_date, flagged) values (?, ?, ?, 0)";
        let _ = connect_db().execute(stmt, params![user_id, msg.content, Utc::now().to_rfc3339()]);
        HttpResponse::NoContent().json("")
    }
    else {
        HttpResponse::NotFound().json("")
    }
}

#[get("fllws/{username}")]
async fn follows_get(path: web::Path<(String,)>, amount: web::Query<MessageAmount>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let conn = connect_db();
        let mut stmt = conn.prepare("
            SELECT user.username FROM user
            INNER JOIN follower ON follower.whom_id=user.user_id
            WHERE follower.who_id=?
            LIMIT ?").unwrap();
        let mut rows = stmt.query([user_id, amount.no]).unwrap();

        let mut followers: Vec<String> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            followers.push(row.get(0).unwrap());
        }

        HttpResponse::Ok().json(Follows{follows: followers})
    }
    else {
        HttpResponse::NotFound().json("")
    }
}

#[post("fllws/{username}")]
async fn follows_post(path: web::Path<(String,)>, follow_param: web::Json<FollowParam>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let follow_param = follow_param.into_inner();
        if let Some(follow_username) = follow_param.follow {
            if let Some(follow_user_id) = get_user_id(&follow_username) {
                let _ = connect_db().execute("INSERT INTO follower (who_id, whom_id) VALUES (?, ?)", [user_id, follow_user_id]).unwrap();
                return HttpResponse::NoContent()
            }
        } else if let Some(unfollow_username) = follow_param.unfollow {
            if let Some(unfollow_user_id) = get_user_id(&unfollow_username) {
                let _ = connect_db().execute("DELETE FROM follower WHERE who_id=? and WHOM_ID=?", [user_id, unfollow_user_id]).unwrap();
                return HttpResponse::NoContent()
            }
        }

        HttpResponse::BadRequest()
    }
    else {
        HttpResponse::NotFound()
    }
}
