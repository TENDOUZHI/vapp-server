use chrono::NaiveDate;
use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct ProgramsResponse {
    pub message: String,
    pub list: Option<Vec<Programs>>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Programs {
    pub id: i32,
    pub user_id: i32,
    // pub data: Option<String>,
    pub name: Option<String>,
    pub lastdate: Option<NaiveDate>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ProgramsDataResponse {
    pub msg: String,
    pub data: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ProgramsData {
    pub id: i32,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ProgramInsert {
    pub user_id: i32,
    pub name: String,
    pub lastdate: NaiveDate
}

#[derive(Debug,Deserialize)]
pub struct ProgramDelete {
    pub id: i32,
    pub user_id:i32
}
#[derive(Debug,Deserialize,sqlx::FromRow)]
pub struct ProgramSave{
    pub id: i32,
    pub user_id: i32,
    pub data: String
}