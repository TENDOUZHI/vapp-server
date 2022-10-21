use std::env;

use actix::{Actor, StreamHandler, WrapFuture, ContextFutureSpawner};
use actix_web_actors::ws;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

use super::programs_handler::programs_save_handler;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramsResponse {
    pub message: String,
    pub list: Option<Vec<Programs>>,
}

#[derive(Debug, Deserialize)]
pub struct ProgramList {
    pub user_id: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Programs {
    pub id: i32,
    pub user_id: i32,
    // pub data: Option<String>,
    pub name: Option<String>,
    pub lastdate: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramsDataResponse {
    pub msg: String,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramsData {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramInsert {
    pub user_id: i32,
    pub name: String,
    pub lastdate: NaiveDate,
}

#[derive(Debug, Deserialize)]
pub struct ProgramDelete {
    pub id: i32,
    pub user_id: i32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ProgramSave {
    pub id: i32,
    pub user_id: i32,
    pub data: String,
    pub program_name: String,
    pub lastdate: NaiveDate,
}

impl Actor for ProgramSave {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ProgramSave {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let mut state = 0;
                let mut comma_number = 0;
                let mut user_id_vec: Vec<char> = vec![];
                let mut program_id_vec: Vec<char> = vec![];
                let mut program_name_vec: Vec<char> = vec![];
                let mut lastdate_vec: Vec<char> = vec![];
                let mut length = 0;
                for (i, char) in text.chars().into_iter().enumerate() {
                    if char == ';' {
                        state = 0;
                        comma_number += 1;
                    }
                    if state == 1 && comma_number == 0 {
                        user_id_vec.push(char);
                    } else if state == 1 && comma_number == 1 {
                        program_id_vec.push(char);
                    } else if state == 1 && comma_number == 2 {
                        program_name_vec.push(char);
                    } else if state == 1 && comma_number == 3 {
                        lastdate_vec.push(char);
                    }
                    if char == ':' && comma_number == 0 {
                        state = 1;
                    }
                    if char == ':' && comma_number == 1 {
                        state = 1;
                    }
                    if char == ':' && comma_number == 2 {
                        state = 1;
                    }
                    if char == ':' && comma_number == 3 {
                        state = 1;
                    }
                    if comma_number == 4 {
                        // println!("{i}");
                        length = i;
                        break;
                    }
                }
                self.data = text.to_string().split_off(length + 1);
                self.user_id = user_id_vec
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                self.id = program_id_vec
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                self.program_name = program_name_vec.iter().collect::<String>();
                self.lastdate = Local::today().naive_local();
                let new_program_save = ProgramSave {
                    id: self.id,
                    user_id: self.user_id,
                    data: self.data.clone(),
                    program_name: self.program_name.clone(),
                    lastdate: self.lastdate.clone(),
                };
                let res = async move {
                    let connect_str = env::var("DATABASE_URL").expect("geting database env url");
                    let pool = PgPoolOptions::new()
                        .max_connections(5)
                        .connect(&connect_str).await.unwrap();
                    programs_save_handler(&pool, &new_program_save).await.unwrap();
                };
                // await websocket
                res.into_actor(self).spawn(ctx);
                // return message
                ctx.text("save successfully")
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
