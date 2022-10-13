use sqlx::{query, Pool, Postgres};

use super::ast::{ProgramInsert, Programs, ProgramsResponse};

pub async fn programs_handler(pool: &Pool<Postgres>) -> Result<ProgramsResponse, String> {
    let res = query!("select * from programs").fetch_all(pool).await;
    match res {
        Ok(v) => {
            let mut list: Vec<Programs> = vec![];
            for i in v {
                list.push(Programs {
                    id: i.id,
                    user_id: i.user_id,
                    data: i.data,
                    name: i.name,
                    lastdate: i.lastdate,
                })
            }
            Ok(ProgramsResponse {
                message: "select program list successfully".to_string(),
                list: Some(list),
            })
        }
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn programs_insert_handler(pool: &Pool<Postgres>, info: &ProgramInsert) -> Result<String,String> {
    let res = query!(
        r#"
        insert into programs(user_id,name,lastdate) values($1,$2,$3)
    "#,
        info.user_id,
        info.name,
        info.lastdate
    )
    .fetch_all(pool)
    .await;
    match res {
        Ok(_) => {
            Ok("insert programs successfully".to_string())
        },
        Err(e) => Err(format!("{e}")),
    }
}
