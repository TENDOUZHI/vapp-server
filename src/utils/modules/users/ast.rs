use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct LoginPassword {
    pub username: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub message: Option<String>,
    pub emessage:Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct LoginResponse {
    pub status: u16,
    pub message: String,
    pub id: Option<i32>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub avatar: Option<Vec<u8>>,
    pub email: Option<String>,
    pub telephone: Option<String>
}
#[derive(Deserialize,Debug)]
pub struct LoginVerify {
    pub id: i32,
    pub username: String,
    pub token: String
}


#[derive(Debug)]
pub enum LoginType {
    Name,
    Tel,
    Email,
    Message,
    Emessage
}

#[derive(Deserialize,Serialize,Debug)]
pub struct VerifyCode {
    pub email_address: String,
    pub is_login: bool
}

#[derive(Debug)]
pub enum CodeType {
    Login,
    Register
}

#[derive(Deserialize,Debug)]
pub struct UpdateUserName {
    pub user_id: i32,
    pub username: String
}

#[derive(Deserialize,Debug)]
pub struct UpdateMail {
    pub user_id: i32,
    pub mail: String,
    pub passcode: String,
    pub password: String
}

#[derive(Deserialize,Debug)]
pub struct UpdateTel {
    pub user_id: i32,
    pub telephone: String,
    pub password: String
}

#[derive(Deserialize,Debug)]
pub struct DisBind {
    pub user_id: i32
}