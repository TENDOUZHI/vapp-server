use std::path::{Path};

use super::renderer::initial_project;
use crate::utils::{
    ast::{Info, Vapp},
    renderer::parse_vapp, compress::compress,
};
use actix_files::{ NamedFile};
use actix_web::{get, post, web::Json, HttpResponse, Responder, Result};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
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
    let raw_path = format!("mini/{}.zip",&project_name);
    let path = Path::new(&raw_path);
    parse_vapp(info.into_inner());
    compress(project_name);
    Ok(NamedFile::open(path).expect("return file"))
}
