use actix_web::{ get, App, HttpServer, Responder};
use actix_files as fs;
use askama_actix::{Template};
use chrono::{DateTime,Utc};
use rusqlite::{ Connection, Result};

#[derive(Template)] // this will generate the code...
#[template(path = "../templates/hello.html")] // using the template in this path, relative
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
struct User {
    user_id: i64,
    user_name: String,
} 
struct Messages {
    text: String,
    email:String,
    username: String,
    pub_date: DateTime<Utc>,
}
// https://doc.rust-lang.org/std/vec/index.html
// https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Template)] // this will generate the code...
#[template(path = "../templates/simpleTemplate.html")] // using the template in this path, relative
struct SimpleTemplate {// should be used as a wrapper not sure how
    messages: Vec<Messages> //Option with messages aka options(vec) or just a vec
}

struct LayoutTemplateOld<'a> {// should be used as a wrapper not sure how
    title: &'a str,
    body: &'a str,
    g: Option<G>,// Option is a nullable field user not defined
    flashes: Option<Vec<Messages>> //Option with messages aka options(vec) or just a vec
}
// String
struct G {
    db: Connection,
    user: User,
}
//#[derive(Template)]
//#[template(path = "../templates/timeline.html")] 
struct TimelineTemplate<'a> {
    name: String, // Is it not title 
    messages:Vec<Messages>,// Vec<Message>, dynamic array of message structs 
    user: Option<User>,
    request_endpoint: &'a str,//just an URL does not need to be strict
    profile_user: Option<User>,
    followed: bool,//Unsure how to define this properly
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(timeline)
            .service(fs::Files::new("/static", "../static").index_file("index.html"))
            
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}

fn get_database_string() -> String {
    "/tmp/mini-x.db".to_string()
}

fn connect_db() ->  Result<Connection>  {
    Connection::open(get_database_string())
}

fn init_db()  -> rusqlite::Result<()>{
    let schema_sql = std::fs::read_to_string("schema.sql").unwrap();
    let conn = connect_db().unwrap();

    conn.execute_batch(&schema_sql)?;
    Ok(())
}

fn query_db(query: &str) {
    
}

fn get_user_id(username: &str ) {

}

fn g_mock() -> Result<G, rusqlite::Error> {
    let connection = Connection::open_in_memory()?;
    Ok(G {
        db: connection,
        user: User {
            user_id: 1,
            user_name: String::from("Test Name"),
        },
    })
}

#[get("/")]
async fn timeline() -> impl Responder {
    let messages: Vec<Messages> = vec![
        Messages {
            text: String::from("Hello, world!"),
            email: String::from("example@email.com"),
            username: String::from("user123"),
            pub_date: Utc::now(),
        },
        Messages {
            text: String::from("How are you?"),
            email: String::from("another@email.com"),
            username: String::from("user456"),
            pub_date: Utc::now(),
        },
    ];

    return SimpleTemplate { messages };
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
    return HelloTemplate { name: "AAAA" };
}

#[get("/{username}/unfollow")]
async fn unfollow_user() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/add_message")]
async fn add_message() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/login")]
async fn login() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/register")]
async fn register() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/logout")]
async fn logout() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn render_hello_template() -> impl Responder {
    game();
    return HelloTemplate { name: "AAAA" };
}

async fn cookie_test(session: Session) -> impl Responder {
    if let Ok(Some(count)) = session.get::<i32>("counter") {
        session.insert("counter", count + 1);
    } else {
        session.insert("counter", 0);
    }
    
    let count = session.get::<i32>("counter").unwrap().unwrap();
    HttpResponse::Ok().body(format!("Session has been refreshed {count} times"))
}

fn game() -> Result<()> {
    let conn = Connection::open("/tmp/test.db")?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
} 
}