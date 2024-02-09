use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use askama_actix::{Template};

#[derive(Template)] // this will generate the code...
#[template(path = "../templates/hello.html")] // using the template in this path, relative
struct HelloTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}
#[derive(Template)] // this will generate the code...
#[template(path = "../templates/timeline.html")] 
struct TimelineTemplate<'a> { // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
    profile_user: ProfileUser<'a>,
    flashes: bool,
    request: RequestStruct,
    
}
struct ProfileUser<'a> {
    username: &'a str,
}

struct RequestStruct {
    endpoint: str,
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
