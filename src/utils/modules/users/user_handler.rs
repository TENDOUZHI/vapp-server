use actix_session::Session;
use actix_web::HttpResponse;
use crypto::{digest::Digest, sha1::Sha1};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use sqlx::{query, Pool, Postgres};
use std::io::{Error, ErrorKind};

use crate::utils::jwt::jwt::{check_token, genarate_token};

use super::ast::{
    CodeType, LoginPassword, LoginResponse, LoginType, LoginVerify, UpdateMail, UpdateTel,
    UpdateUserName, DisBind,
};

// pub fn login_response(res: Result<LoginResponse, String>) -> HttpResponse {
//     return match res {
//         Ok(v) => HttpResponse::Ok().body("v"),
//         Err(e) => HttpResponse::from_error(Error::new(ErrorKind::ConnectionRefused, e)),
//     };
// }

pub fn register_response(res: Result<String, String>) -> HttpResponse {
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
) -> Result<LoginResponse, String> {
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
                    let token = genarate_token(info.username.as_ref().unwrap().to_string());
                    match token {
                        Ok(token) => Ok(LoginResponse {
                            status: 200,
                            message: "username login successfully".to_string(),
                            id: Some(res[0].id),
                            token: Some(token),
                            username: Some(res[0].username.clone()),
                            avatar: res[0].avatar.clone(),
                            email: res[0].email.clone(),
                            telephone: res[0].telephone.clone(),
                        }),
                        Err(_) => Err("token genarate error".to_owned()),
                    }
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
                    let token = genarate_token(info.telephone.as_ref().unwrap().to_string());
                    match token {
                        Ok(token) => Ok(LoginResponse {
                            status: 200,
                            message: "telephone login successfully".to_string(),
                            id: Some(res[0].id),
                            token: Some(token),
                            username: Some(res[0].username.clone()),
                            avatar: res[0].avatar.clone(),
                            email: res[0].email.clone(),
                            telephone: res[0].telephone.clone(),
                        }),
                        Err(_) => Err("token genarate error".to_owned()),
                    }
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
                    let token = genarate_token(info.email.as_ref().unwrap().to_string());
                    match token {
                        Ok(token) => Ok(LoginResponse {
                            status: 200,
                            message: "email login successfully".to_string(),
                            id: Some(res[0].id),
                            token: Some(token),
                            username: Some(res[0].username.clone()),
                            avatar: res[0].avatar.clone(),
                            email: res[0].email.clone(),
                            telephone: res[0].telephone.clone(),
                        }),
                        Err(_) => Err("token genarate error".to_owned()),
                    }
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
                    let token = genarate_token(info.email.as_ref().unwrap().to_string());
                    match token {
                        Ok(token) => Ok(LoginResponse {
                            status: 200,
                            message: "email login successfully".to_string(),
                            id: Some(res[0].id),
                            token: Some(token),
                            username: Some(res[0].username.clone()),
                            avatar: res[0].avatar.clone(),
                            email: res[0].email.clone(),
                            telephone: res[0].telephone.clone(),
                        }),
                        Err(_) => Err("token genarate error".to_owned()),
                    }
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
            if res.len() == 1 {
                // verify the code is correct
                let code = session
                    .get::<String>(&email)
                    .expect("get verify code from redis");
                if code == info.emessage {
                    let token = genarate_token(info.email.as_ref().unwrap().to_string());
                    match token {
                        Ok(token) => Ok(LoginResponse {
                            status: 200,
                            message: "email login successfully".to_string(),
                            id: Some(res[0].id),
                            token: Some(token),
                            username: Some(res[0].username.clone()),
                            avatar: res[0].avatar.clone(),
                            email: res[0].email.clone(),
                            telephone: res[0].telephone.clone(),
                        }),
                        Err(_) => Err("token genarate error".to_owned()),
                    }
                } else {
                    Err("email verify code incorrect".to_string())
                }
            } else {
                Err("email incorrect".to_string())
            }
        }
    }
}

pub async fn login_verify_handler(
    pool: &Pool<Postgres>,
    info: &LoginVerify,
) -> Result<LoginResponse, String> {
    let validate = check_token(info.token.clone());
    if validate {
        let res = query!("select * from users where id=$1", info.id)
            .fetch_all(pool)
            .await;
        match res {
            Ok(row) => {
                let token = genarate_token(info.username.clone());
                match token {
                    Ok(v) => Ok(LoginResponse {
                        status: 200,
                        message: "token validate".to_string(),
                        id: Some(row[0].id),
                        token: Some(v),
                        username: Some(row[0].username.clone()),
                        avatar: row[0].avatar.clone(),
                        email: row[0].email.clone(),
                        telephone: row[0].telephone.clone(),
                    }),
                    Err(e) => Err(format!("{e}")),
                }
            }
            Err(e) => Err(format!("{e}")),
        }
    } else {
        Err("invalide token".to_string())
    }
}

pub async fn register_handler(
    pool: &Pool<Postgres>,
    info: &LoginPassword,
    login_type: LoginType,
    session: Session,
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
                    info.username,
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
            let email = info.email.as_ref().unwrap().clone();
            let exist = sqlx::query!(
                r#"
                select * from users where email=$1
            "#,
                info.email
            )
            .fetch_all(pool)
            .await
            .expect("this email has existed");
            // no same email has registed
            let code = session.get::<String>(&email).expect("msg");
            if code == info.emessage {
                if exist.len() == 0 {
                    let password = password_crypto(info.password.as_ref().unwrap());
                    sqlx::query!(
                        r#"
                    insert into users(username,email,password) values($1,$2,$3)
                "#,
                        info.username,
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
            } else {
                Err("email verify code incorrect".to_string())
            }
        }
        _ => Err("not support this register way".to_string()),
    }
}

pub async fn update_username_handler(
    pool: &Pool<Postgres>,
    info: &UpdateUserName,
) -> Result<String, String> {
    let res = query!(
        "
        select * from users where id=$1
    ",
        info.user_id
    )
    .fetch_all(pool)
    .await;
    match res {
        Ok(row) => {
            if row.len() == 1 {
                let update_res = query!(
                    "
    update users set username=$1 where id=$2
    ",
                    info.username,
                    info.user_id
                )
                .fetch_all(pool)
                .await;
                match update_res {
                    Ok(_) => Ok("update username successfully".to_string()),
                    Err(e) => Err(format!("{e}")),
                }
            } else {
                Err("user id is invalidate".to_string())
            }
        }
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn update_mail_handler(
    pool: &Pool<Postgres>,
    info: &UpdateMail,
    session: Session,
) -> Result<String, String> {
    let res = query!(
        "
        select * from users where id=$1
    ",
        info.user_id
    )
    .fetch_all(pool)
    .await;
    match res {
        Ok(v) => {
            if v.len() == 1 {
                let passowrd = password_crypto(&info.password);
                let code = session
                    .get::<String>(&info.mail)
                    .expect("get verify code from redis");
                if code.unwrap() == info.passcode && passowrd == v[0].password {
                    let update_res = query!(
                        "
                        update users set email=$1 where id=$2
                        ",
                        info.mail,
                        info.user_id
                    )
                    .fetch_all(pool)
                    .await;
                    match update_res {
                        Ok(_) => Ok("update mail successfully".to_string()),
                        Err(e) => Err(format!("{e}")),
                    }
                } else {
                    Err("passcode or password incorrect".to_string())
                }
            } else {
                Err("user_id is invalidate".to_string())
            }
        }
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn update_tel_handler(pool: &Pool<Postgres>, info: &UpdateTel) -> Result<String, String> {
    let res = query!(
        "
        select * from users where id=$1
    ",
        info.user_id
    )
    .fetch_all(pool)
    .await;
    match res {
        Ok(v) => {
            if v.len() == 1 {
                let passowrd = password_crypto(&info.password);
                if passowrd == v[0].password {
                    let update_res = query!(
                        "
                    update users set telephone=$1 where id=$2
                    ",
                        info.telephone,
                        info.user_id
                    )
                    .fetch_all(pool)
                    .await;
                    match update_res {
                        Ok(_) => Ok("update telephone successfully".to_string()),
                        Err(e) => Err(format!("{e}")),
                    }
                } else {
                    Err("password is incorrect".to_string())
                }
            } else {
                Err("user id is invalidate".to_string())
            }
        }
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn disbind_mail_handler(pool: &Pool<Postgres>,info: &DisBind) -> Result<String,String> {
    let res = query!("
        update users set email='' where id=$1
    ",info.user_id).fetch_all(pool).await;
    match res {
        Ok(_) => Ok("disbind email successfully".to_string()),
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn disbind_tel_handler(pool: &Pool<Postgres>,info: &DisBind) -> Result<String,String> {
    let res = query!("
        update users set telephone='' where id=$1
    ",info.user_id).fetch_all(pool).await;
    match res {
        Ok(_) => Ok("disbind telephone successfully".to_string()),
        Err(e) => Err(format!("{e}")),
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
        "<h3>Here is your {} approval code: {}</h3>
        <div style='width=100%;
        height=300px;
        background=pink;'>{}</div>
        ",
        types.to_lowercase(),
        verify_code,
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
        "atgkszdqiffxecih".to_string(),
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
