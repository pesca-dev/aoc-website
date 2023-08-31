use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::repository::UserRepository;

use super::Session;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub sessions: Vec<Session>,
}

impl User {
    pub async fn create(self) -> Result<(), Box<dyn Error>> {
        let User {
            username,
            email,
            password,
            ..
        } = self;
        UserRepository::create(username, password, email).await?;

        Ok(())
    }

    pub async fn get_by_id(id: impl ToString) -> Option<User> {
        let Some(user) = UserRepository::get_by_id(id).await.ok().flatten() else {
            return None;
        };

        let id = user.id().expect("user from database should have id");
        let UserRepository {
            username,
            password,
            email,
            email_verified,
            ..
        } = user;

        Some(Self {
            id,
            username,
            password,
            email,
            email_verified,
            sessions: vec![],
        })
    }

    pub async fn get_by_username(username: impl ToString) -> Option<User> {
        let Some(user) = UserRepository::get_by_username(username)
            .await
            .ok()
            .flatten()
        else {
            return None;
        };

        let id = user.id().expect("user from database should have id");
        let UserRepository {
            username,
            password,
            email,
            email_verified,
            ..
        } = user;

        Some(Self {
            id,
            username,
            password,
            email,
            email_verified,
            sessions: vec![],
        })
    }

    pub async fn verify_email(&self) {
        if let Err(e) = UserRepository::verify_email(&self.id).await {
            tracing::error!(
                "failed to verify email for user '{user_id}': {e:?}",
                user_id = self.id
            );
        }
    }

    pub async fn login(&mut self) -> Option<String> {
        let Some(session) = Session::new(self).await else {
            tracing::error!("failed to login user ({})", self.id);
            return None;
        };

        let id = session.id.clone();

        self.sessions.push(session);

        Some(id)
    }
}
