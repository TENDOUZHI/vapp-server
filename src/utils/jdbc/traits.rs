use actix_web::http::Error;
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::Postgres;
use super::tabels::InstancePool;
use std::result::Result;



#[async_trait]
pub trait HooksFn {
    fn create(&self, pool: &Pool<Postgres>);
    fn update(&self, pool: &Pool<Postgres>);
    async fn select(&self, pool: &Pool<Postgres>) -> Result<InstancePool,Error>;
    fn insert(&self, pool: &Pool<Postgres>);
    fn delete(&self, pool: &Pool<Postgres>);
}


#[async_trait]
pub trait Create {
    fn create();
}

