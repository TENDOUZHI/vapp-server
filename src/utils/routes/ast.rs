use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct LoginPassword {
    pub username: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
    pub message: Option<String>,
    pub emessage:Option<String>,
    pub password: String,
}

#[derive(Debug)]
pub enum LoginType {
    Name,
    Tel,
    Email,
    Message,
    Emessage
}