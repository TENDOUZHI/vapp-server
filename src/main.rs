use actix_web::{App, HttpServer, web};
mod utils;
use actix_cors::Cors;
use actix_web::http::header;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utils::{lib::{hello, vapp}, jdbc::{users::Hooks, traits::Instance}};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connect_str = env::var("DATABASE_URL").expect("geting database env url");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connect_str)
        .await
        .unwrap();
    // let list = sqlx::query!("select * from users").fetch_all(&pool).await.unwrap();
    // for ls in list {
    //     println!("{:?}",ls);
    // }
    
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(vapp)
            .service(hello)
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
