use std::fmt::Display;

use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::{
    model::User,
    services::{jwt, jwt::VerifyJWT},
};

#[cfg(feature = "ssr")]
use super::{create_jwt, send_verification_mail};

#[derive(Serialize, Deserialize)]
pub enum VerificationResult {
    Ok,
    InvalidToken,
    ExpiredToken,
    InternalServerError,
}

impl Display for VerificationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationResult::Ok => f.write_str("Success!"),
            VerificationResult::InvalidToken => f.write_str("Invalid Token Provided!"),
            VerificationResult::ExpiredToken => f.write_str("Expired Token Provided!"),
            VerificationResult::InternalServerError => f.write_str("Internal Server Error!"),
        }
    }
}

#[tracing::instrument(level = "trace", skip(cx))]
#[server(Verify, "/api")]
pub async fn verify_user(cx: Scope, token: String) -> Result<VerificationResult, ServerFnError> {
    let payload: VerifyJWT = match jwt::extract(token) {
        Ok(data) => data,
        Err(e) => {
            tracing::warn!("failed to extract JWT: {e:#?}");
            return Ok(VerificationResult::InvalidToken);
        }
    };

    let timestamp = payload.exp;
    let now = chrono::Utc::now().timestamp();

    let is_valid = (now - timestamp) > 0;
    if !is_valid {
        // TODO: add message for that
        return Ok(VerificationResult::ExpiredToken);
    }

    let username = payload.sub;
    let user = match User::get_by_username(&username).await {
        Some(user) => user,
        None => {
            return Ok(VerificationResult::InvalidToken);
        }
    };

    user.verify_email().await;

    Ok(VerificationResult::Ok)
}

#[tracing::instrument(level = "trace", skip(cx))]
#[server(ResendVerification, "/api")]
pub async fn resend_verification_mail(cx: Scope, username: String) -> Result<(), ServerFnError> {
    let Some(user) = User::get_by_username(&username).await else {
        return Ok(());
    };

    let Ok(token) = create_jwt(&username) else {
        return Ok(());
    };

    let _ = send_verification_mail(username, user.email, token);
    Ok(())
}
