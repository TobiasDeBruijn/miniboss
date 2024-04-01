use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::{migrate, MySqlPool};
use std::ops::Deref;

pub use sqlx::Error;

const MIGRATOR: Migrator = migrate!();

#[derive(Debug, Clone)]
pub struct Database(MySqlPool);

impl Database {
    pub async fn new(u: &str, p: &str, h: &str, d: &str) -> sqlx::Result<Self> {
        let pool = MySqlPool::connect_with(
            MySqlConnectOptions::new()
                .username(u)
                .password(p)
                .database(d)
                .host(h),
        )
            .await?;

        MIGRATOR.run(&pool).await?;
        Ok(Self(pool))
    }
}

impl Deref for Database {
    type Target = MySqlPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}