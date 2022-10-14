use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use sqlx::PgPool;

use crate::utils::modules::programs::{
    ast::{ProgramDelete, ProgramInsert, ProgramsResponse, ProgramsData, ProgramsDataResponse},
    programs_handler::{programs_delete_handler, programs_handler, programs_insert_handler, programs_data_handler},
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

#[post("/programlist/data")]
pub async fn programdata(pool: web::Data<PgPool>, payload: Json<ProgramsData>) -> impl Responder {
    let info = payload.into_inner();
    let res = programs_data_handler(&pool, &info).await;
    match res {
        Ok(v) => web::Json(v),
        Err(e) => {
            let msg = ProgramsDataResponse {
                msg: e,
                data: None
            };
            web::Json(msg)
        }
    }
}

#[post("/programlist/insert")]
pub async fn programs_insert(
    pool: web::Data<PgPool>,
    payload: Json<ProgramInsert>,
) -> impl Responder {
    let info = payload.into_inner();
    let res = programs_insert_handler(&pool, &info).await;
    match res {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::Forbidden().body(e),
    }
}

#[post("/programlist/delete")]
pub async fn programs_delete(
    pool: web::Data<PgPool>,
    payload: Json<ProgramDelete>,
) -> impl Responder {
    let info = payload.into_inner();
    let res = programs_delete_handler(&pool, &info).await;
    match res {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::Forbidden().body(e),
    }
}
