use crate::utils::jwt::jwt::check_token;
use actix_session::Session;
use actix_web::{
    get, post,
    // web,
    Error, HttpResponse, Responder, Result,
};
// use sqlx::PgPool;

#[get("/hello")]
pub async fn hello() -> Result<HttpResponse, Error> {
    // pool: web::Data<PgPool>, session: Session
    let vali = check_token("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJWYXBwX0FjdGl4Iiwic3ViIjoiTm9ybWFsIHVzZXJzIiwiYXVkIjoid3p5IiwiZXhwIjoxNjY1MjM4MzI5NjA4LCJpYXQiOjE2NjUyMzgyNjk2MDgsIm5iZiI6MTY2NTIzODI2OTYwOH0.yF0ImMA7DckG4srAatmruOEKOcc3xQZYOoK-cbhTgOc".to_string());
    println!("{vali}");
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

// #[post("/vnode")]
// pub async fn vnode(info: Json<Info>) -> impl Responder {
//     initial_project(info.into_inner());
//     HttpResponse::Ok().body("we accepted it")
// }

// #[post("/vapp")]
// pub async fn vapp(info: Json<Vapp>) -> Result<NamedFile> {
//     let project_name = &info.project_name.clone();
//     let raw_path = format!("mini/{}.zip", &project_name);
//     let path = Path::new(&raw_path);
//     parse_vapp(info.into_inner());
//     compress(project_name);
//     Ok(NamedFile::open(path).expect("return file"))
// }
