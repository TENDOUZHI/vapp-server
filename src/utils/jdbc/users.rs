use super::{
    tabels::{InstanceEn, InstancePool, Users},
    traits::HooksFn,
};
use actix_web::http::Error;
use async_trait::async_trait;
use sqlx::{query, Pool, Postgres};
use std::result::Result;

#[derive(Debug)]
pub struct Hooks;

// #[async_trait]
// impl HooksFn for Hooks {
//     fn create(&self, pool: &Pool<Postgres>) {}
//     fn update(&self, pool: &Pool<Postgres>) {}
//     async fn select(&self, pool: &Pool<Postgres>) -> Result<InstancePool, Error> {
//         let users = query!("select * from users")
//             .fetch_all(pool)
//             .await
//             .expect("select users");
//         let mut user_info: Vec<Users> = vec![];
//         for user in users {
//             user_info.push(Users {
//                 id: user.id,
//                 username: user.username,
//                 password: user.password,
//                 telephone: user.telephone,
//                 avatar: user.avatar,
//                 email: user.email,
//             });
//         }
//         Ok(InstanceEn::Users(user_info))
//     }
//     fn insert(&self, pool: &Pool<Postgres>) {}
//     fn delete(&self, pool: &Pool<Postgres>) {}
// }
