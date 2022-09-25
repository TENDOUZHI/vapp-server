use actix_web::{ web, App, HttpServer};
mod utils;
use utils::lib::{hello,echo,vnode, vapp};
use actix_cors::Cors;
use actix_web::http::header;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(vnode)
            .service(vapp)
            .wrap( Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}



