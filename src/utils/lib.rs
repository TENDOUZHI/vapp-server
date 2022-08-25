use std::{any::Any, fs};

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
#[derive(Deserialize, Debug,Clone,Entity)]
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

#[derive(Entity,Debug)]
struct Book {
    id: u64,
    title:String,
    pages: u64,
    author: String
}
impl Book {
    fn to_vec(&self) {
        println!("{:?}",self);
    }
}



#[post("/vnode")]
pub async fn vnode(info: Json<Info>) -> impl Responder {
    // initial_project(info.into_inner());
    let select_sql = Book::select();
    let size = Book::size();
    let book = Book{
        id:21,
        title:"sss".to_string(),
        pages:642,
        author:"ooo".to_string()
    };
    // println!("{:?}",select_sql);
    // println!("{:?}",size);
    println!("{:?}",book.to_vec());
    HttpResponse::Ok().body("we accepted it")
}
