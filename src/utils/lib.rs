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

impl Style {
    pub fn to_style_sheet(&self) -> String {
        self.parser()
    }

    fn parser(&self) -> String {
        let vec_style = self.vec_style();
        let mut style_line = String::from("");
        for (name, value) in vec_style {
            style_line = format!("{}{}:{};",style_line,name,value);
            // println!("{}:{};",name,value)
        }
        style_line
    }

    fn vec_style(&self) -> Vec<(&str, &String)> {
        let mut style_vec = vec![];
        style_vec.push(("width",&self.width));
        style_vec.push(("height",&self.height));
        style_vec.push(("fontSize",&self.fontSize));
        style_vec.push(("color",&self.color));
        style_vec.push(("marginTop",&self.marginTop));
        style_vec.push(("marginBottom",&self.marginBottom));
        style_vec.push(("marginLeft",&self.marginLeft));
        style_vec.push(("marginRight",&self.marginRight));
        style_vec.push(("paddingTop",&self.paddingTop));
        style_vec.push(("paddingBottom",&self.paddingBottom));
        style_vec.push(("paddingLeft",&self.paddingLeft));
        style_vec.push(("paddingRight",&self.paddingRight));
        style_vec.push(("borderRadius",&self.borderRadius));
        style_vec.push(("borderWidth",&self.borderWidth));
        style_vec.push(("borderColor",&self.borderColor));
        style_vec.push(("backgroundColor",&self.backgroundColor));
        style_vec.push(("opacity",&self.opacity));
        style_vec.push(("display",&self.display));
        style_vec.push(("flexDirection",&self.flexDirection));
        style_vec.push(("justifyContent",&self.justifyContent));
        style_vec.push(("justifyItems",&self.justifyItems));
        style_vec.push(("alignContent",&self.alignContent));
        style_vec.push(("alignItems",&self.alignItems));
        style_vec
    }


}



#[post("/vnode")]
pub async fn vnode(info: Json<Info>) -> impl Responder {
    initial_project(info.into_inner());
    // let sy = info.into_inner().style;
    HttpResponse::Ok().body("we accepted it")
}
