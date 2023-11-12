use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::hooks::use_database;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct UserRepository {
    #[serde(skip_serializing)]
    id: Option<Thing>,
    pub username: String,
    pub password: String,
    pub email: String,
    pub email_verified: bool,
}

impl UserRepository {
    pub const TABLE: &'static str = "user";

    pub fn id(&self) -> Option<String> {
        self.id.as_ref().map(|id| format!("{}:{}", id.tb, id.id))
    }

    #[tracing::instrument(level = "trace")]
    pub async fn get_all() -> Result<Vec<UserRepository>, surrealdb::Error> {
        tracing::debug!("getting all users from the database");
        let db = use_database().await;

        db.select(Self::TABLE).await
    }

    #[tracing::instrument(level = "trace")]
    pub async fn get_by_id(id: &str) -> Result<Option<UserRepository>, surrealdb::Error> {
        tracing::debug!(
            "trying to get user '{id}' from the database",
            id = id.to_string()
        );
        let db = use_database().await;

        let result: Option<UserRepository> = db.select((Self::TABLE, id.to_string())).await?;

        Ok(result)
    }

    #[tracing::instrument(level = "trace")]
    pub async fn get_by_username(
        username: &str,
    ) -> Result<Option<UserRepository>, surrealdb::Error> {
        tracing::debug!(
            "trying to get user '{username}' from the database",
            username = username.to_string()
        );
        let db = use_database().await;

        let mut result = db
            .query("SELECT * FROM type::table($table) where username = $username;")
            .bind(("table", Self::TABLE))
            .bind(("username", username.to_string()))
            .await?;

        result.take(0)
    }

    #[tracing::instrument(level = "trace")]
    pub async fn verify_email(user_id: &str) -> Result<(), surrealdb::Error> {
        tracing::debug!("verify email in DB for '{user_id}'");
        let db = use_database().await;

        db.query(format!("UPDATE {user_id} SET email_verified = true"))
            .await?;
        Ok(())
    }

    #[tracing::instrument(level = "trace")]
    pub async fn create(
        username: String,
        password: String,
        email: String,
    ) -> Result<Option<UserRepository>, surrealdb::Error> {
        tracing::debug!("creating user in database");
        let db = use_database().await;
        let result: Option<UserRepository> = db
            .create(Self::TABLE)
            .content(UserRepository {
                username,
                password,
                email,
                email_verified: false,
                ..Default::default()
            })
            .await?
            .first()
            .cloned();

        Ok(result)
    }
}
