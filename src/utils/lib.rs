use std::path::Path;
use super::renderer::initial_project;
use crate::utils::{
    ast::{Info, Vapp},
    compress::compress,
    renderer::parse_vapp,
};
use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Json},
    Error, HttpResponse, Responder, Result,
};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use sqlx::PgPool;

#[get("/hello")]
pub async fn hello(pool: web::Data<PgPool>, session: Session) -> Result<HttpResponse, Error> {
    let mut hasher = Sha256::new();
    let text = String::from("56231");
    hasher.input_str(&text);
    println!("{} => {}",text,hasher.result_str());
    Ok(HttpResponse::Ok().body("get"))
}
#[post("/echo")]
pub async fn echo(req_body: String, session: Session) -> impl Responder {
    // if let Some(count) = session.get::<i32>("counter").expect("no session") {
    //     session.insert("counter", count + 1).expect("add one");
    // } else {
    //     session.insert("counter", 1).expect("insert");
    // }
    let s = session.get::<String>("counter").unwrap();
    println!("{:?}", s);
    HttpResponse::Ok().body(req_body)
}

#[post("/vnode")]
pub async fn vnode(info: Json<Info>) -> impl Responder {
    initial_project(info.into_inner());
    HttpResponse::Ok().body("we accepted it")
}

#[post("/vapp")]
pub async fn vapp(info: Json<Vapp>) -> Result<NamedFile> {
    let project_name = &info.project_name.clone();
    let raw_path = format!("mini/{}.zip", &project_name);
    let path = Path::new(&raw_path);
    parse_vapp(info.into_inner());
    compress(project_name);
    Ok(NamedFile::open(path).expect("return file"))
}
