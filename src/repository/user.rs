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
    pub const TABLE: &str = "user";

    pub fn id(&self) -> Option<String> {
        self.id.as_ref().map(|id| format!("{}:{}", id.tb, id.id))
    }

    pub async fn get_all() -> Result<Vec<UserRepository>, surrealdb::Error> {
        let db = use_database().await;

        db.select(Self::TABLE).await
    }

    pub async fn get_by_id(id: impl ToString) -> Result<Option<UserRepository>, surrealdb::Error> {
        let db = use_database().await;

        let result: Option<UserRepository> = db.select((Self::TABLE, id.to_string())).await?;

        Ok(result)
    }

    pub async fn get_by_username(
        username: impl ToString,
    ) -> Result<Option<UserRepository>, surrealdb::Error> {
        let db = use_database().await;

        let mut result = db
            .query("SELECT * FROM type::table($table) where username = $username;")
            .bind(("table", Self::TABLE))
            .bind(("username", username.to_string()))
            .await?;

        result.take(0)
    }

    pub async fn verify_email(user_id: &str) -> Result<(), surrealdb::Error> {
        let db = use_database().await;

        db.query(format!("UPDATE {user_id} SET email_verified = true"))
            .await?;
        Ok(())
    }

    pub async fn create(
        username: String,
        password: String,
        email: String,
    ) -> Result<Option<UserRepository>, surrealdb::Error> {
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
            .await?;

        // TODO: this should be changed in beta9
        Ok(result)
    }
}
