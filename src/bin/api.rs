
mod api_structs;
use api_structs::structs::*;
use std::sync::Mutex;
use actix_files as fs;
use actix_web::web;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use pwhash::bcrypt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let latest = web::Data::new(LatestAction {latest: Mutex::new(-1)});
    if !std::fs::metadata(get_database_string()).is_ok() {
        let _ = init_db();
    }
    HttpServer::new(move || {
        App::new()
            .app_data(latest.clone())
            .service(fs::Files::new("/static", "./static/").index_file("index.html"))
            .service(get_latest)
            .service(post_register)
            .service(messages_per_user_get)
            .service(messages_per_user_post)
            .service(messages_api)
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}

fn get_database_string() -> String {
    String::from("/tmp/mini-x_api.db")
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

    let _ = connect_db().execute(
        "insert into user (
            username, email, pw_hash) values (?, ?, ?)",
        params![info.username, info.email, hash ],
    );

    
    if let Some(err_msg) = error {
        let reg_err = RegisterError {status: 400, error_msg: err_msg.to_string()};
        HttpResponse::BadRequest().json(reg_err)
    }
    else {
        HttpResponse::NoContent().json(String::from(""))
    }
    
}

#[get("/msgs")]
async fn messages_api(msgs: web::Query<MessagesQuery>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder{
    update_latest(query, latest_action);

    let conn = connect_db();
    let mut stmt = conn.prepare("
        SELECT message.*, user.* FROM message, user
        WHERE message.flagged = 0 AND message.author_id = user.user_id
        ORDER BY message.pub_date DESC LIMIT ?").unwrap();
    let result = stmt.query_map([msgs.no], |row| { 
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
async fn messages_per_user_get(path: web::Path<(String,)>, msgs: web::Query<MessagesQuery>, query: web::Query<Latest>, latest_action: web::Data<LatestAction>) -> impl Responder {
    update_latest(query, latest_action);
    let username = &path.0;
    if let Some(user_id) = get_user_id(username) {
        let conn = connect_db();
        let mut stmt = conn.prepare("
            SELECT message.*, user.* FROM message, user
            WHERE message.flagged = 0 AND
            user.user_id = message.author_id AND user.user_id = ?
            ORDER BY message.pub_date DESC LIMIT ?").unwrap();
        let result = stmt.query_map([user_id, msgs.no], |row| { 
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