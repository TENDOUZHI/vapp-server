use std::any::Any;

use super::{parser::parser,renderer::initial_project};
use actix_web::{ get, post, web::Json, HttpResponse, Responder};
use serde::Deserialize;
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub style: Style,
    pub content: Option<String>,
    pub children: Vec<Info>,
}
#[derive(Deserialize, Debug)]
pub struct Style {
    pub width: String,
    pub height: String,
    pub fontSize: String,
    pub color: String,
    pub marginTop: String,
    pub marginBottom: String,
    pub marginLeft: String,
    pub marginRight: String,
    pub paddingTop: String,
    pub paddingBottom: String,
    pub paddingLeft: String,
    pub paddingRight: String,
    pub borderRadius: String,
    pub borderWidth: String,
    pub borderColor: String,
    pub backgroundColor: String,
    pub opacity: String,
    pub display: String,
    pub flexDirection: String,
    pub justifyContent: String,
    pub justifyItems: String,
    pub alignContent: String,
    pub alignItems: String,
}
#[post("/vnode")]
pub async fn vnode(info: Json<Info>) -> impl Responder {
    initial_project(info.into_inner());
    // parser(info.into_inner());
    HttpResponse::Ok().body("we accepted it")
}
