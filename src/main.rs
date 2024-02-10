use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use askama_actix::{Template};

#[derive(Template)] // this will generate the code...
#[template(path = "../templates/hello.html")] // using the template in this path, relative
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
struct User {
    user_id: String,
    user_name: String,
} 
// https://doc.rust-lang.org/std/vec/index.html
// https://doc.rust-lang.org/std/option/enum.Option.html
#[derive(Template)] // this will generate the code...
#[template(path = "../templates/layout.html")] // using the template in this path, relative
struct LayoutTemplate<'a> {// should be used as a wrapper not sure how
    title: &'a str,
    body: &'a str,
    g_user: Option<User>,// Optione is a nullable field user not defined
    flashes: None//Option with messeges aka options(vec) or just a vec
}
#[derive(Template)]
#[template(path = "../templates/timeline.html")] 
struct TimelineTemplate<'a> {
    name: String, // Is it not title 
    messages:Vec<String>,// Vec<Message>, dynamic array of mesege structs 
    user: Option<User>,
    request_endpoint: &'a str,//just an URL does not need to be strct
    profile_user: Option<User>,
    followed: bool,//Unsure how to difine this properly
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
    .bind(("127.0.0.1", 8080))?
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
    return HelloTemplate { name: "AAAA" };
}
