use leptos::*;

use crate::{model::User, repository::LoggedInRepository};

use super::use_identity;

pub async fn use_user(cx: Scope) -> Option<User> {
    let Ok(identity) = use_identity(cx) else {
        error!("failed to get identity");
        return None;
    };

    let Ok(session_id) = identity.id() else {
        error!("failed to get session id!");
        return None;
    };

    LoggedInRepository::find_user_via_session(&session_id).await
}
