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
}

impl UserRepository {
    const TABLE: &str = "user";

    pub fn id(&self) -> Option<String> {
        self.id.as_ref().map(|id| format!("{}:{}", id.tb, id.id))
    }

    pub async fn get_all() -> Result<Vec<UserRepository>, surrealdb::Error> {
        let db = use_database("aoc-website").await;

        db.select(Self::TABLE).await
    }

    pub async fn get_by_id(id: impl ToString) -> Result<Option<UserRepository>, surrealdb::Error> {
        let db = use_database("aoc-website").await;

        let mut result = db
            .query("SELECT * FROM type::table($table) where id = $id;")
            .bind(("table", Self::TABLE))
            .bind(("id", id.to_string()))
            .await?;

        result.take(0)
    }

    pub async fn get_by_username(
        username: impl ToString,
    ) -> Result<Option<UserRepository>, surrealdb::Error> {
        let db = use_database("aoc-website").await;

        let mut result = db
            .query("SELECT * FROM type::table($table) where username = $username;")
            .bind(("table", Self::TABLE))
            .bind(("username", username.to_string()))
            .await?;

        result.take(0)
    }

    pub async fn create(
        username: String,
        password: String,
        email: String,
    ) -> Result<Option<UserRepository>, surrealdb::Error> {
        let db = use_database("aoc-website").await;
        let result: Vec<UserRepository> = db
            .create("user")
            .content(UserRepository {
                username,
                password,
                email,
                ..Default::default()
            })
            .await?;

        // TODO: this should be changed in beta9
        Ok(result.get(0).cloned())
    }
}
