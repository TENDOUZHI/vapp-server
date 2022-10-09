use actix_web::dev::Service;
use actix_web::middleware;
use actix_web::{cookie::time, web, App, HttpServer};
mod utils;
use actix_cors::Cors;
use actix_session::CookieSession;
// ,features=["redis-actor-session","cookie-session","redis-rs-session"]
// use actix_redis::RedisSession;
// use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::http::header;
use dotenv::dotenv;
use futures::FutureExt;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utils::{
    lib::{echo, hello},
    routes::user_route::{email_pass_code, login, register},
    vapp::vapp_route::vapp,
};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // coonect to postgres
    let connect_str = env::var("DATABASE_URL").expect("geting database env url");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connect_str)
        .await
        .unwrap();

    // use session middleware
    // let secret_key = Key::generate();
    // let redis_connection_string = "127.0.0.1:6379";

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(vapp)
            .service(hello)
            .service(login)
            .service(register)
            .service(echo)
            .service(email_pass_code)
            // .wrap_fn(|req, srv| {
            //     if req.path() != "/login" || req.path() != "/register" {
            //         println!("yes");
            //         // println!("payload: {:?}",req.parts_mut())
            //         req.into_parts().1.into();
            //     }
            //     println!("Hi from start. You requested: {}", req.path());
            //     srv.call(req).map(|res| {
            //         println!("Hi from response");
            //         res
            //     })
            // })
            .wrap(
                CookieSession::signed(&[0; 32])
                    .secure(false)
                    .expires_in_time(time::Duration::minutes(2)),
            )
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:5173")
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
