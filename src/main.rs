use actix_web::{App, HttpServer};
mod utils;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::http::header;
use utils::lib::vapp;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(vapp)
            .service(
                fs::Files::new("/static", "mini/New Project/")
                    .show_files_listing()
                    .index_file("pages/")
                    .prefer_utf8(true)
                    .use_etag(false)
                    .use_last_modified(true),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
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
