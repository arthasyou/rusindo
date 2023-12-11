mod settings;
use crate::settings::Settings;
use dotenv::dotenv;
use rusindo::database::mongo::MongoDB;
use rusindo::network::web::server;

use actix_web::{get, post, web, web::Data, web::ServiceConfig, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(db: Data<MongoDB>) -> impl Responder {
    println!("{:?}", db);
    HttpResponse::Ok().body("Hey there!")
}

pub fn root(cfg: &mut ServiceConfig) {
    cfg.route("/hey", web::get().to(manual_hello));
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let db = MongoDB::new(settings.mongodb).await.unwrap();

    // println!("{:?}", db);
    server::start(settings.web, root, vec![db]).await
}
