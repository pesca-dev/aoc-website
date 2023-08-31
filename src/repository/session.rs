use chrono::Utc;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{thing, Thing};

use crate::hooks::use_database;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct SessionRepository {
    #[serde(skip_serializing)]
    id: Option<Thing>,
    created_at: String,
}

impl SessionRepository {
    const TABLE: &str = "session";

    pub fn id(&self) -> Option<String> {
        self.id.as_ref().map(|id| format!("{}:{}", id.tb, id.id))
    }

    pub async fn create() -> Result<Option<SessionRepository>, surrealdb::Error> {
        let db = use_database().await;
        let result: Option<SessionRepository> = db
            .create(Self::TABLE)
            .content(SessionRepository {
                created_at: Utc::now().to_rfc3339(),
                ..Default::default()
            })
            .await?;

        Ok(result)
    }

    pub async fn delete(id: &str) -> Result<(), surrealdb::Error> {
        let Ok(Thing { tb, id }) = thing(id) else {
            return Ok(());
        };

        let db = use_database().await;

        let _: Option<SessionRepository> = db.delete((tb, id)).await?;
        Ok(())
    }
}
