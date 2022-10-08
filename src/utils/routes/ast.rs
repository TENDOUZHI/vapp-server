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
    pub message: String,
    pub token: Option<String>
}

pub struct ErrorMessage{
    pub msg: String
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