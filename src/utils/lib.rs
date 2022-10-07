use std::path::Path;

use super::renderer::initial_project;
use crate::utils::{
    ast::{Info, Vapp},
    compress::compress,
    jdbc::{traits::HooksFn, users::Hooks},
    renderer::parse_vapp,
};
use actix_files::NamedFile;
use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Json},
    Error, HttpResponse, Responder, Result,
};
use lettre::{
    transport::smtp::authentication::Credentials, Address, Message, SmtpTransport, Transport,
};
use sqlx::PgPool;

#[get("/hello")]
pub async fn hello(pool: web::Data<PgPool>, session: Session) -> Result<HttpResponse, Error> {
    // let hooks = Hooks;
    // let user = hooks.select(&pool).await.unwrap();
    // println!("{:?}",user);
    let m = Message::builder()
        .from("Vapp <2649821154@qq.com>".parse().unwrap())
        .reply_to("User <2649821154@qq.com>".parse().unwrap())
        .to("Reciver <2112087898@qq.com>".parse().unwrap())
        .subject("hello")
        .body("This is a test email".to_string())
        .unwrap();
    let creds = Credentials::new(
        "2649821154@qq.com".to_string(),
        "vmaudkfypicvdiif".to_string(),
    );
    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&m) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    };
    if let Some(count) = session::get::<i32>("counter").unwrap() {
        
    }
    Ok(HttpResponse::Ok().body("Hello world!"))
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
    let raw_path = format!("mini/{}.zip", &project_name);
    let path = Path::new(&raw_path);
    parse_vapp(info.into_inner());
    compress(project_name);
    Ok(NamedFile::open(path).expect("return file"))
}
