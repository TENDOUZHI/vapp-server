use sqlx::{query, Pool, Postgres};

use super::ast::{
    ProgramDelete, ProgramInsert, ProgramSave, Programs, ProgramsData, ProgramsDataResponse,
    ProgramsResponse,
};

pub async fn programs_handler(pool: &Pool<Postgres>) -> Result<ProgramsResponse, String> {
    let res = query!("select id, user_id, name, lastdate from programs")
        .fetch_all(pool)
        .await;
    match res {
        Ok(v) => {
            let mut list: Vec<Programs> = vec![];
            for i in v {
                list.push(Programs {
                    id: i.id,
                    user_id: i.user_id,
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

pub async fn programs_data_handler(
    pool: &Pool<Postgres>,
    info: &ProgramsData,
) -> Result<ProgramsDataResponse, String> {
    let res = query!(r#"select data from programs where id=$1"#, info.id)
        .fetch_one(pool)
        .await;
    match res {
        Ok(v) => Ok(ProgramsDataResponse {
            msg: "select program data successfully".to_string(),
            data: v.data,
        }),
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn programs_insert_handler(
    pool: &Pool<Postgres>,
    info: &ProgramInsert,
) -> Result<String, String> {
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
        Ok(_) => Ok("insert programs successfully".to_string()),
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn programs_delete_handler(
    pool: &Pool<Postgres>,
    info: &ProgramDelete,
) -> Result<String, String> {
    let ensure = query!(
        r#"
        select id from programs where id=$1 and user_id=$2
    "#,
        info.id,
        info.user_id
    )
    .fetch_all(pool)
    .await;
    match ensure {
        Ok(v) => {
            if v.len() == 1 {
                let res = query!(
                    r#"
        delete from programs where id=$1 and user_id=$2
    "#,
                    info.id,
                    info.user_id
                )
                .fetch_all(pool)
                .await;
                match res {
                    Ok(_) => {
                        // println!("{}", v.len());
                        Ok("delete programs successfully".to_string())
                    }
                    Err(e) => Err(format!("{e}")),
                }
            } else {
                Err("program id or user_id invalide".to_string())
            }
        }
        Err(e) => Err(format!("{e}")),
    }
}

pub async fn programs_save_handler(pool: &Pool<Postgres>, info: &ProgramSave) -> Result<String,String> {
    let res = query!(
        r#"
        update programs set data=$1 where id=$2 and user_id=$3
    "#,
    &info.data,
    info.id,
    info.user_id
    )
    .fetch_all(pool)
    .await;
    // println!("{}",&info.data);
    match res {
        Ok(_) => {
            Ok("update value successfully".to_string())
        },
        Err(e) => {
            Err(format!("{e}"))
        }
    }
}
