use std::fmt::Result;

use super::traits::Instance;
use actix_web::http::Error;
use async_trait::async_trait;
use sqlx::{query, Pool, Postgres};

#[derive(Debug)]
pub struct Hooks;

#[derive(Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub telephone: Option<i64>,
    pub avatar: Option<Vec<u8>>,
}

#[async_trait]
impl Instance for Hooks {
    fn create(&self, pool: &Pool<Postgres>) {}
    fn update(&self, pool: &Pool<Postgres>) {}
    async fn select(&self, pool: &Pool<Postgres>) {
        let users = query!("select * from users")
            .fetch_all(pool)
            .await
            .expect("select users");
        let mut user_info: Vec<Users> = vec![];
        for user in users {
            user_info.push(Users {
                id: user.id,
                username: user.username,
                password: user.password,
                telephone: user.telephone,
                avatar: user.avatar,
            });
        }
    }
    fn insert(&self, pool: &Pool<Postgres>) {}
    fn delete(&self, pool: &Pool<Postgres>) {}
}
