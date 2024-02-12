use actix_files as fs;
use actix_web::web;
use actix_web::{post, get, http, App, HttpServer,HttpResponse, Responder, cookie::Key};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use askama_actix::Template;
use actix_web_flash_messages::{
    FlashMessage, IncomingFlashMessages,
};
use actix_web_flash_messages::{FlashMessagesFramework, storage::CookieMessageStore};
use chrono::{DateTime, Utc, Duration};
use rusqlite::{Connection, Result, params};
use uuid::Uuid;
use serde::Deserialize;

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
}
// https://doc.rust-lang.org/std/vec/index.html
// https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Template)] // this will generate the code...
#[template(path = "../templates/simpleTemplate.html")] // using the template in this path, relative
struct SimpleTemplate<'a> {
    // should be used as a wrapper not sure how
    messages: Vec<Messages>, //Option with messages aka options(vec) or just a vec
    request_endpoint: &'a str,
    profile_user: Option<User>,
    user: Option<User>,
    //g: Option<G>,
    followed: bool, //Unsure how to define this properly
    flashes: Vec<String>
}

//#[derive(Template)]
//#[template(path = "../templates/timeline.html")]
struct TimelineTemplate<'a> {
    name: String,            // Is it not title
    messages: Vec<Messages>, // Vec<Message>, dynamic array of message structs
    user: Option<User>,
    request_endpoint: &'a str, //just an URL does not need to be strict
    profile_user: Option<User>,
    followed: bool, //Unsure how to define this properly
}

#[derive(Template)]
#[template(path = "../templates/login.html")]
struct LoginTemplate {
    user: Option<User>,
    //g: Option<G>,
    flashes: Vec<String>,
    username: String,            // Is it not title
    error: String,            // Is it not title
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let signing_key = Key::generate(); // This will usually come from configuration!
    let message_store = CookieMessageStore::builder(signing_key).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./static/").index_file("index.html"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                .cookie_secure(false)
                .build()
            )
            .wrap(message_framework.clone())
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

fn connect_db() -> Result<Connection> {
    Connection::open(get_database_string())
}

fn init_db() -> rusqlite::Result<()> {
    let schema_sql = std::fs::read_to_string("schema.sql").unwrap();
    let conn = connect_db().unwrap();

    conn.execute_batch(&schema_sql)?;
    Ok(())
}

fn query_db(query: &str) {}

fn get_user_id(username: &str) -> Result<usize, rusqlite::Error>{
    let conn = connect_db().unwrap();
    conn.execute("SELECT user_id FROM user WHERE username = ?1", params![username])
}

fn g(session: Session) -> Result<()> {
    let connection = connect_db()?;
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

fn get_messages() -> Vec<Messages> {
    vec![
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
    ]
}

#[get("/")]
async fn timeline(flash_messages: IncomingFlashMessages) -> impl Responder {

    //if no user
    //    Ok(HttpResponse::SeeOther()
    // .header("Location", "/public")
    // .finish());
    let g_mock = g_mock().unwrap();
    return SimpleTemplate { 
        messages: get_messages(), 
        request_endpoint:"/aa", 
        profile_user: Some(User {user_id:Uuid::new_v4(), username:String::from("Name") }), 
        user: Some(g_mock.user ), 
        followed: false,
        flashes: get_flashes(flash_messages),
    };
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
    return HelloTemplate { name: "AAAA" }
}

#[post("/add_message")]
async fn add_message() -> impl Responder {
    return HelloTemplate { name: "AAAA" };
}

#[get("/login")]
async fn login(flash_messages: IncomingFlashMessages) -> impl Responder {
    FlashMessage::info("You were logged in!!").send();
    /* HttpResponse::TemporaryRedirect()
        .insert_header((http::header::LOCATION, "/"))
        .finish() */
    let g_mock = g_mock().unwrap();
    return LoginTemplate { 
        user: Some(g_mock.user ), 
        flashes: get_flashes(flash_messages),
        error: String::from(""),
        username: String::from("a")
    };
}
#[derive(Deserialize)]
struct Info {
    username: String,
}

#[post("/login")]
async fn post_login( info: web::Form<Info>) -> impl Responder {
    println!("{}", info.username);

    HelloTemplate { name: "RegisterPage" }
}

fn get_flashes(messages: IncomingFlashMessages) -> Vec<String> {
    messages.iter().map(|m : &FlashMessage| -> String {m.content().to_string()}).collect()
}

#[get("/register")]
async fn register() -> impl Responder {
    HelloTemplate { name: "RegisterPage" }
}

#[get("/logout")]
async fn logout() -> impl Responder {
    HelloTemplate { name: "LogoutPage" } 
    // fn game() -> Result<()> {
    //     let conn = Connection::open("/tmp/test.db")?;

    //     conn.execute(
    //         "CREATE TABLE person (
    //               id              INTEGER PRIMARY KEY,
    //               name            TEXT NOT NULL,
    //               data            BLOB
    //               )",
    //         [],
    //     )?;
    //     let me = Person {
    //         id: 0,
    //         name: "Steven".to_string(),
    //         data: None,
    //     };
    //     conn.execute(
    //         "INSERT INTO person (name, data) VALUES (?1, ?2)",
    //         params![me.name, me.data],
    //     )?;

    //     let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    //     let person_iter = stmt.query_map([], |row| {
    //         Ok(Person {
    //             id: row.get(0)?,
    //             name: row.get(1)?,
    //             data: row.get(2)?,
    //         })
    //     })?;

    //     for person in person_iter {
    //         println!("Found person {:?}", person.unwrap());
    //     }

    //     Ok(())
    // }
}
