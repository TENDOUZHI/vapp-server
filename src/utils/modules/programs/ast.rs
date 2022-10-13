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
    pub data: Option<String>,
    pub name: Option<String>,
    pub lastdate: Option<NaiveDate>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct InsertProgram {
    pub user_id: i32,
    pub name: String,
    pub lastdate: NaiveDate
}