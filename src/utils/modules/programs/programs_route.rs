use actix_web::{post, Responder, HttpResponse, web};
use sqlx::PgPool;

#[post("/replist")]
pub async fn replist(pool: web::Data<PgPool>,) -> impl Responder {

    HttpResponse::Ok().body("replist")
}
