use sqlx::{query, Pool, Postgres};

pub async fn replist_handler(pool: &Pool<Postgres>) {
    let res = query!("select * from programs")
        .fetch_all(pool)
        .await
        .expect("select programs list");
    if res.len() != 0 {
        
    }
}
