use std::fmt::Result;
use actix_web::http::Error;
use async_trait::async_trait;
use sqlx::Pool;
use sqlx::Postgres;

#[async_trait]
pub trait Instance {
    fn create(&self, pool: &Pool<Postgres>);
    fn update(&self, pool: &Pool<Postgres>);
    async fn select(&self, pool: &Pool<Postgres>);
    fn insert(&self, pool: &Pool<Postgres>);
    fn delete(&self, pool: &Pool<Postgres>);
}
