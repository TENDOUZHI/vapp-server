use actix_web::{
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponse, Responder, ResponseError, Result,
};
use sqlx::{PgPool, Pool, Postgres};

use crate::utils::{
    handler::user_handler::{login_handler, login_response},
    routes::ast::{LoginPassword, LoginType},
};

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, payload: Json<LoginPassword>) -> impl Responder {
    let info = payload.into_inner();
    if let Some(_) = info.username {
        let res = login_handler(&pool, &info, LoginType::Name).await;
        return login_response(res);
    } else if let Some(_) = info.email {
        if let Some(_) = info.emessage {
            let res = login_handler(&pool, &info, LoginType::Emessage).await;
            return login_response(res);
        }
        let res = login_handler(&pool, &info, LoginType::Email).await;
        return login_response(res);
    } else if let Some(_) = info.telephone {
        if let Some(_) = info.message {
            let res = login_handler(&pool, &info, LoginType::Message).await;
            return login_response(res);
        }
        let res = login_handler(&pool, &info, LoginType::Tel).await;
        return login_response(res);
    } else {
        HttpResponse::Ok()
            .status(StatusCode::FORBIDDEN)
            .body("login failed")
    }
}
