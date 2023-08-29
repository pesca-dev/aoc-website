use std::error::Error;

use crate::repository::UserRepository;

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
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
            ..
        } = user;

        Some(Self {
            id,
            username,
            password,
            email,
        })
    }
}
