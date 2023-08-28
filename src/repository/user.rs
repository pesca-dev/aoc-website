use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::hooks::use_database;

// TODO: maybe create this via a macro?
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserCreateData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    id: Thing,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    const TABLE: &str = "user";

    pub fn id(&self) -> String {
        format!("{}:{}", self.id.tb, self.id.id)
    }

    pub async fn get_all() -> Result<Vec<User>, surrealdb::Error> {
        let db = use_database("aoc-website").await;

        db.select(Self::TABLE).await
    }

    pub async fn get_by_username(
        username: impl ToString,
    ) -> Result<Option<User>, surrealdb::Error> {
        let db = use_database("aoc-website").await;

        let mut result = db
            .query("SELECT * FROM type::table($table) where username = $username;")
            .bind(("table", Self::TABLE))
            .bind(("username", username.to_string()))
            .await?;

        result.take(0)
    }

    pub async fn create(data: UserCreateData) -> Result<(), surrealdb::Error> {
        let db = use_database("aoc-website").await;
        let _: Vec<User> = db.create(Self::TABLE).content(data).await?;

        Ok(())
    }
}
