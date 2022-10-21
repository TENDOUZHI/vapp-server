use actix_web::{
    get, post,
    web::{self, Json},
    Error, HttpRequest, HttpResponse, Responder,
};
use actix_web_actors::ws;
use chrono::NaiveDate;
use sqlx::PgPool;

use crate::utils::modules::programs::{
    ast::{
        ProgramDelete, ProgramInsert, ProgramSave, ProgramsData, ProgramsDataResponse,
        ProgramsResponse, ProgramList,
    },
    programs_handler::{
        programs_data_handler, programs_delete_handler, programs_handler, programs_insert_handler,
        programs_save_handler,
    },
};

#[post("/programlist")]
pub async fn programlist(pool: web::Data<PgPool>, payload: Json<ProgramList>) -> impl Responder {
    let info = payload.into_inner();
    let res = programs_handler(&pool,&info).await;
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
            let msg = ProgramsDataResponse { msg: e, data: None };
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

#[post("/programlist/save")]
pub async fn programs_save(pool: web::Data<PgPool>, payload: Json<ProgramSave>) -> impl Responder {
    let info = payload.into_inner();
    let res = programs_save_handler(&pool, &info).await;
    match res {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::Forbidden().body(e),
    }
}

#[get("/program/ws")]
pub async fn program_websocket(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let program_save = ProgramSave {
        id: 0,
        user_id: 0,
        program_name: "".to_string(),
        data: "".to_string(),
        lastdate: NaiveDate::from_ymd(2022, 1, 1),
    };
    ws::start(program_save, &req, stream)
}
