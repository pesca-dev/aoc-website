use leptos::*;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{hooks::use_database, repository::SessionRepository};

use super::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

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

impl Session {
    pub async fn find_by_id(id: &str) -> Option<Session> {
        let Some(user) = Self::find_user_via_session(id).await else {
            return None;
        };

        Some(Session {
            id: id.to_string(),
            user_id: user.id,
        })
    }

    pub async fn find_user_via_session(session_id: &str) -> Option<User> {
        let db = use_database().await;

        let Ok(mut response) = db
            .query(format!(
                "select id, <-logged_in<-user as users from {session_id};"
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

    pub async fn new(user: &User) -> Option<Session> {
        let Some(session) = SessionRepository::create().await.ok().flatten() else {
            return None;
        };

        let session_id = session.id().expect("session from DB should have ID");

        let db = use_database().await;

        if let Err(e) = db
            .query(format!("RELATE {}->logged_in->{}", user.id, session_id))
            .await
        {
            error!(
                "Error creating a relation between user ({}) and session ({}): {e:?}",
                user.id, session_id
            );
            if let Err(e) = SessionRepository::delete(&session_id).await {
                error!("Error deleting session ({session_id}): {e:?}");
            };
            return None;
        };

        Some(Session {
            id: session_id,
            user_id: user.id.clone(),
        })
    }

    pub async fn destroy(session_id: &str) {
        if let Err(e) = SessionRepository::delete(&session_id).await {
            error!("Error deleting session ({session_id}): {e:?}");
        };
    }
}
