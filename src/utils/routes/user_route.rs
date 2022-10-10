use actix_session::Session;
use actix_web::{
    http::StatusCode,
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::PgPool;

use crate::utils::{
    handler::user_handler::{email_send, login_handler, register_handler, register_response},
    routes::ast::{CodeType, LoginPassword, LoginResponse, LoginType, VerifyCode},
};

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    payload: Json<LoginPassword>,
    session: Session,
) -> impl Responder {
    let info = payload.into_inner();
    if let Some(_) = info.username {
        let res = login_handler(&pool, &info, LoginType::Name, session).await;
        match res {
            Ok(v) => web::Json(v),
            Err(e) => {
                let msg = LoginResponse {
                    message: e,
                    token: None,
                    username: None,
                    avatar: None,
                    email: None,
                    telephone: None,
                };
                web::Json(msg)
            }
        }
    } else if let Some(_) = info.email {
        if let Some(_) = info.emessage {
            let res = login_handler(&pool, &info, LoginType::Emessage, session).await;
            match res {
                Ok(v) => return web::Json(v),
                Err(e) => {
                    let msg = LoginResponse {
                        message: e,
                        token: None,
                        username: None,
                        avatar: None,
                        email: None,
                        telephone: None,
                    };
                    return web::Json(msg);
                }
            }
        }
        let res = login_handler(&pool, &info, LoginType::Email, session).await;
        match res {
            Ok(v) => web::Json(v),
            Err(e) => {
                let msg = LoginResponse {
                    message: e,
                    token: None,
                    username: None,
                    avatar: None,
                    email: None,
                    telephone: None,
                };
                web::Json(msg)
            }
        }
    } else if let Some(_) = info.telephone {
        if let Some(_) = info.message {
            let res = login_handler(&pool, &info, LoginType::Message, session).await;
            match res {
                Ok(v) => return web::Json(v),
                Err(e) => {
                    let msg = LoginResponse {
                        message: e,
                        token: None,
                        username: None,
                        avatar: None,
                        email: None,
                        telephone: None,
                    };
                    return web::Json(msg);
                }
            }
        }
        let res = login_handler(&pool, &info, LoginType::Tel, session).await;
        match res {
            Ok(v) => web::Json(v),
            Err(e) => {
                let msg = LoginResponse {
                    message: e,
                    token: None,
                    username: None,
                    avatar: None,
                    email: None,
                    telephone: None,
                };
                web::Json(msg)
            }
        }
    } else {
        let msg = LoginResponse {
            message: "login failed".to_string(),
            token: None,
            username: None,
            avatar: None,
            email: None,
            telephone: None,
        };
        web::Json(msg)
    }
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    payload: Json<LoginPassword>,
    session: Session,
) -> impl Responder {
    let info = payload.into_inner();
    // if let Some(_) = info.username {
    //     let res = register_handler(&pool, &info, LoginType::Name,session).await;
    //     return register_response(res);
    // } else
    if let Some(_) = info.email {
        let res = register_handler(&pool, &info, LoginType::Email, session).await;
        return register_response(res);
    } else if let Some(_) = info.telephone {
        let res = register_handler(&pool, &info, LoginType::Tel, session).await;
        return register_response(res);
    } else {
        HttpResponse::Ok()
            .status(StatusCode::FORBIDDEN)
            .body("register failed")
    }
}

#[post("/passcode")]
pub async fn email_pass_code(payload: Json<VerifyCode>, session: Session) -> impl Responder {
    let info = payload.into_inner();
    let address = info.email_address;
    let is_login = info.is_login;
    // println!("{:?}", session.entries());
    let verify_code: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    session
        .insert(&address, &verify_code)
        .expect("storage email verify");
    if is_login {
        email_send(&address, &verify_code, CodeType::Login);
    } else {
        email_send(&address, &verify_code, CodeType::Register);
    }

    HttpResponse::Ok().body("verify code send successfully")
}
