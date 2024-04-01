use sqlx::{FromRow, Result};
use thiserror::Error;

use crate::driver::Database;
use crate::generate_string;
use crate::hash::{hash, verify};

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Error)]
pub enum HashingError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    BCrypt(#[from] bcrypt::BcryptError),
}

impl User {
    pub async fn new(
        driver: &Database,
        name: String,
        email: String,
        is_admin: bool,
    ) -> Result<Self> {
        let user_id = generate_string(32);

        sqlx::query("INSERT INTO users (user_id, name, email, is_admin) VALUES (?, ?, ?, ?)")
            .bind(&user_id)
            .bind(&name)
            .bind(&email)
            .bind(is_admin)
            .execute(&**driver)
            .await?;

        Ok(Self {
            name,
            user_id,
            email,
            is_admin,
        })
    }

    pub async fn set_password(&self, password: &str, pepper: &str, driver: &Database) -> std::result::Result<(), HashingError> {
        let salt = generate_string(16);
        let password = hash(&password, &salt, pepper)?;

        if self.has_password(driver).await? {
            sqlx::query("UPDATE user_credentials SET password = ?, salt = ? WHERE user_id = ?")
                .bind(password)
                .bind(salt)
                .bind(&self.user_id)
                .execute(&**driver)
                .await?;
        } else {
            sqlx::query("INSERT INTO user_credentials (user_id, password, salt) VALUES (?, ?, ?)")
                .bind(&self.user_id)
                .bind(password)
                .bind(salt)
                .execute(&**driver)
                .await?;
        }

        Ok(())
    }

    async fn has_password(&self, driver: &Database) -> Result<bool> {
        Ok(
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user_credentials WHERE user_id = ? LIMIT 1)")
                .bind(&self.user_id)
                .fetch_one(&**driver)
                .await?
        )
    }

    pub async fn verify_password(&self, password: &str, pepper: &str, driver: &Database) -> std::result::Result<bool, HashingError> {
        if !self.has_password(&driver).await? {
            return Ok(false)
        }

        let stored_password: String = sqlx::query_scalar("SELECT password FROM users WHERE user_id = ?")
            .bind(&self.user_id)
            .fetch_one(&**driver)
            .await?;

        Ok(verify(&stored_password, password, pepper)?)
    }

    pub async fn get_by_id(driver: &Database, id: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE user_id = ?")
            .bind(id)
            .fetch_optional(&**driver)
            .await?)
    }

    pub async fn get_by_email(driver: &Database, email: &str) -> Result<Option<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&**driver)
            .await?)
    }

    pub async fn list(driver: &Database) -> Result<Vec<Self>> {
        Ok(sqlx::query_as("SELECT * FROM users")
            .fetch_all(&**driver)
            .await?)
    }

    pub async fn list_permitted_scopes(&self, driver: &Database) -> Result<Vec<String>> {
        Ok(
            sqlx::query_scalar("SELECT scope FROM user_permitted_scopes WHERE user_id = ?")
                .bind(&self.user_id)
                .fetch_all(&**driver)
                .await?,
        )
    }

    pub async fn remove_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_permitted_scopes WHERE user_id = ? AND scope = ?")
            .bind(&self.user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }

    pub async fn grant_permitted_scope(&self, driver: &Database, scope: &str) -> Result<()> {
        sqlx::query("INSERT INTO user_permitted_scopes (user_id, scope) VALUES (?, ?)")
            .bind(&self.user_id)
            .bind(scope)
            .execute(&**driver)
            .await?;

        Ok(())
    }
}