use leptos::*;

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{hooks::use_database, model::User, repository::SessionRepository};

use super::UserRepository;

pub struct LoggedInRepository {}

#[derive(Debug, Serialize, Deserialize)]
struct LoggenInRelation {
    user_id: String,
    session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoggedInModel {
    id: Thing,
    users: Vec<Thing>,
}

impl LoggedInRepository {
    const TABLE: &str = "logged_in";

    pub async fn find_user_via_session(session_id: &str) -> Option<User> {
        let db = use_database().await;

        let Ok(mut response) = db
            .query(format!(
                "select id, <-{relation}<-{user_table} as users from {session_id};",
                relation = Self::TABLE,
                user_table = UserRepository::TABLE
            ))
            .await
        else {
            return None;
        };

        let Ok(Some(result)): Result<Option<LoggedInModel>, surrealdb::Error> = response.take(0)
        else {
            return None;
        };

        let Some(user) = result.users.get(0) else {
            return None;
        };

        User::get_by_id(user.id.clone()).await
    }

    pub async fn attach_user_to_session(user: &str, session: &str) -> Result<(), ()> {
        let db = use_database().await;

        if let Err(e) = db
            .query(format!(
                "RELATE {user}->{table}->{session}",
                table = Self::TABLE
            ))
            .await
        {
            tracing::error!(
                "failed to creat a relation between user ({}) and session ({}): {e:?}",
                user,
                session
            );
            if let Err(e) = SessionRepository::delete(session).await {
                tracing::error!("failed to delete session ({session}): {e:?}");
            };
            return Err(());
        };

        Ok(())
    }
}
