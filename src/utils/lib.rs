use std::{any::Any, fs};

use crate::utils::{ast::{Info, Vapp}, renderer::parse_vapp};

use super::{parser::parser,renderer::initial_project};
use actix_web::{ get, post, web::Json, HttpResponse, Responder};
use serde::Deserialize;
use hello_macro_derive::{self, Entity};
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
    // let sy = info.into_inner().style;
    HttpResponse::Ok().body("we accepted it")
}

#[post("/vapp")]
pub async fn vapp(info: Json<Vapp>) -> impl Responder {
    parse_vapp(info.into_inner());
    // println!("{:?}",info.into_inner());
    // let sy = info.into_inner().style;
    HttpResponse::Ok().body("we accepted it")
}
