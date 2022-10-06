#[derive(Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub telephone: Option<String>,
    pub avatar: Option<Vec<u8>>,
    pub email:Option<String>
}
#[derive(Debug)]
pub enum InstanceEn {
    Users(Vec<Users>)
}

pub type InstancePool = InstanceEn;