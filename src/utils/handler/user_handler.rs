use actix_web::{http::StatusCode, HttpResponse};
use sqlx::{Pool, Postgres};
use std::io::{Error,ErrorKind};

use crate::utils::routes::ast::{LoginPassword, LoginType};

pub fn login_response(res: Result<String, String>) -> HttpResponse {
    return match res {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::from_error(Error::new(ErrorKind::ConnectionRefused, e)),
    };
}

pub async fn login_handler(
    pool: &Pool<Postgres>,
    info: &LoginPassword,
    login_type: LoginType,
) -> Result<String, String> {
    match login_type {
        LoginType::Name => {
            let res = sqlx::query!(
                r#"
                    select * from users where username=$1
            "#,
                info.username
            )
            .fetch_all(pool)
            .await
            .expect("username login");
            if res.len() != 0 {
                if res[0].password == info.password {
                    Ok("username login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("username login faild".to_string())
            }
        }
        LoginType::Tel => {
            let res = sqlx::query!(
                r#"
                    select * from users where telephone=$1
            "#,
                info.telephone
            )
            .fetch_all(pool)
            .await
            .expect("username login");
            if res.len() != 0 {
                if res[0].password == info.password {
                    Ok("telephone login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("telephone login failed".to_string())
            }
        }
        LoginType::Email => {
            let res = sqlx::query!(
                r#"
                    select * from users where email=$1
            "#,
                info.email
            )
            .fetch_all(pool)
            .await
            .expect("username login");
            if res.len() != 0 {
                if res[0].password == info.password {
                    Ok("email login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("email login failed".to_string())
            }
        }
        LoginType::Message => {
            let res = sqlx::query!(
                r#"
                    select * from users where email=$1
            "#,
                info.email
            )
            .fetch_all(pool)
            .await
            .expect("username login");
            if res.len() != 0 {
                if res[0].password == info.password {
                    Ok("email login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("email login failed".to_string())
            }
        },
        LoginType::Emessage => todo!(),
    }
}
