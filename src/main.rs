use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use askama_actix::Template;
use rusqlite::{params, Connection, Result};

#[derive(Template)] // this will generate the code...
#[template(path = "../templates/hello.html")] // using the template in this path, relative

struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
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
            .service(hello)
            .service(fs::Files::new("/static", "../static").index_file("index.html"))
            .route("/hey", web::get().to(manual_hello))
            .route("/test", web::get().to(render_hello_template))
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn render_hello_template() -> impl Responder {
    game();
    return HelloTemplate { name: "AAAA" };
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