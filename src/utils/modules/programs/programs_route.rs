use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use sqlx::PgPool;

use crate::utils::modules::programs::{
    ast::{InsertProgram, ProgramsResponse},
    programs_handler::programs_handler,
};

#[get("/programlist")]
pub async fn programlist(pool: web::Data<PgPool>) -> impl Responder {
    let res = programs_handler(&pool).await;
    match res {
        Ok(v) => web::Json(v),
        Err(e) => {
            let msg = ProgramsResponse {
                message: e,
                list: None,
            };
            web::Json(msg)
        }
    }
}

#[post("/programlist/insert")]
pub async fn program_insert(
    pool: web::Data<PgPool>,
    payload: Json<InsertProgram>,
) -> impl Responder {
    HttpResponse::Ok().body("insert successfully")
}
