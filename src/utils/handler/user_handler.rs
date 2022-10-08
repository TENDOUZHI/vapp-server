use actix_session::Session;
use actix_web::HttpResponse;
use crypto::{digest::Digest, sha1::Sha1};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use sqlx::{Pool, Postgres};
use std::io::{Error, ErrorKind};

use crate::utils::routes::ast::{CodeType, LoginPassword, LoginType};

pub fn login_register_response(res: Result<String, String>) -> HttpResponse {
    return match res {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::from_error(Error::new(ErrorKind::ConnectionRefused, e)),
    };
}

pub async fn login_handler(
    pool: &Pool<Postgres>,
    info: &LoginPassword,
    login_type: LoginType,
    session: Session,
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
                let password = password_crypto(info.password.as_ref().unwrap());
                if res[0].password == password {
                    Ok("username login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("username incorrect".to_string())
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
                let password = password_crypto(info.password.as_ref().unwrap());
                if res[0].password == password {
                    Ok("telephone login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("telephone number incorrect".to_string())
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
            .expect("email login");
            if res.len() != 0 {
                let password = password_crypto(info.password.as_ref().unwrap());
                if res[0].password == password {
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
                let password = password_crypto(info.password.as_ref().unwrap());
                if res[0].password == password {
                    Ok("email login successfully".to_string())
                } else {
                    Err("password incorrect".to_string())
                }
            } else {
                Err("telephone number incorrect".to_string())
            }
        }
        LoginType::Emessage => {
            let email = info.email.as_ref().unwrap().clone();
            // verify is email is correct
            let res = sqlx::query!(
                r#"
                select * from users where email=$1
        "#,
                info.email
            )
            .fetch_all(pool)
            .await
            .expect("email login");
            if res.len() != 0 {
                // verify the code is correct
                let code = session
                    .get::<String>(&email)
                    .expect("get verify code from redis");
                println!("{:?}", code);
                if code == info.emessage {
                    Ok("email login successfully".to_string())
                } else {
                    Err("email verify code incorrect".to_string())
                }
            } else {
                Err("email incorrect".to_string())
            }
        }
    }
}

pub async fn register_handler(
    pool: &Pool<Postgres>,
    info: &LoginPassword,
    login_type: LoginType,
) -> Result<String, String> {
    match login_type {
        LoginType::Name => {
            let exist = sqlx::query!(
                r#"
                select * from users where username=$1
            "#,
                info.username
            )
            .fetch_all(pool)
            .await
            .expect("is name has existed");
            // no same username has registed
            if exist.len() == 0 {
                let password = password_crypto(info.password.as_ref().unwrap());
                sqlx::query!(
                    r#"
                    insert into users(username,password) values($1,$2)
                "#,
                    info.username,
                    password
                )
                .fetch_all(pool)
                .await
                .expect("insert username");
                Ok("username register successfully".to_string())
            } else {
                Err("the username has been used".to_string())
            }
        }
        LoginType::Tel => {
            if info.telephone.as_ref().unwrap().len() != 11 {
                return Err("the length of telephone must be 11".to_string());
            }
            let exist = sqlx::query!(
                r#"
                select * from users where telephone=$1
            "#,
                info.telephone
            )
            .fetch_all(pool)
            .await
            .expect("this telephone has existed");
            // no same username has registed
            if exist.len() == 0 {
                let password = password_crypto(info.password.as_ref().unwrap());
                sqlx::query!(
                    r#"
                    insert into users(username,telephone,password) values($1,$2,$3)
                "#,
                    info.telephone,
                    info.telephone,
                    password
                )
                .fetch_all(pool)
                .await
                .expect("insert telephone");
                Ok("telephone register successfully".to_string())
            } else {
                Err("the telephone has been used".to_string())
            }
        }
        LoginType::Email => {
            let exist = sqlx::query!(
                r#"
                select * from users where email=$1
            "#,
                info.email
            )
            .fetch_all(pool)
            .await
            .expect("this email has existed");
            // no same username has registed
            if exist.len() == 0 {
                let password = password_crypto(info.password.as_ref().unwrap());
                sqlx::query!(
                    r#"
                    insert into users(username,email,password) values($1,$2,$3)
                "#,
                    info.email,
                    info.email,
                    password
                )
                .fetch_all(pool)
                .await
                .expect("insert email");
                Ok("email register successfully".to_string())
            } else {
                Err("the email has been used".to_string())
            }
        }
        _ => Err("not support this register way".to_string()),
    }
}

pub fn email_send(email_address: &str, verify_code: &str, code_type: CodeType) {
    let types;
    match code_type {
        CodeType::Login => types = "Login",
        CodeType::Register => types = "Register",
    }
    let reciver = format!("Reciver <{}>", email_address);
    let title = format!("{types} code: {}", verify_code);
    let content = format!(
        "<h2>Here is your {} approval code: {}</h2>",
        types.to_lowercase(),
        verify_code
    );
    let m = Message::builder()
        .from("Vapp <2649821154@qq.com>".parse().unwrap())
        .reply_to("Vapp <2649821154@qq.com>".parse().unwrap())
        .to(reciver.parse().unwrap())
        .subject(title)
        .body(content)
        .unwrap();
    let creds = Credentials::new(
        "2649821154@qq.com".to_string(),
        "vmaudkfypicvdiif".to_string(),
    );
    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&m) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    };
}

fn password_crypto(password: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(password);
    hasher.result_str()
}
