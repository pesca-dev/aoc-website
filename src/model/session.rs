use leptos::*;
use serde::{Deserialize, Serialize};

use crate::repository::{LoggedInRepository, SessionRepository};

use super::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
}

impl Session {
    #[tracing::instrument(level = "trace")]
    pub async fn find_by_id(id: &str) -> Option<Session> {
        tracing::debug!("get session '{id}'");
        let Some(user) = LoggedInRepository::find_user_via_session(id).await else {
            tracing::debug!("session not found");
            return None;
        };

        Some(Session {
            id: id.to_string(),
            user_id: user.id,
        })
    }

    #[tracing::instrument(level = "trace", skip(user))]
    pub async fn new(user: &User) -> Option<Session> {
        let Some(session) = (match SessionRepository::create().await {
            Ok(session) => session,
            Err(e) => {
                tracing::error!("failed to create session: ({e:#?})");
                return None;
            }
        }) else {
            return None;
        };

        let session_id = session.id().expect("session from DB should have ID");

        if LoggedInRepository::attach_user_to_session(&user.id, &session_id)
            .await
            .is_err()
        {
            tracing::error!(
                "failed to attach user '{user}' to session '{session_id}'",
                user = user.id
            );
            return None;
        };

        Some(Session {
            id: session_id,
            user_id: user.id.clone(),
        })
    }

    #[tracing::instrument(level = "trace")]
    pub async fn destroy(session_id: &str) {
        if let Err(e) = SessionRepository::delete(session_id).await {
            tracing::error!("Error deleting session ({session_id}): {e:?}");
        };
    }
}
