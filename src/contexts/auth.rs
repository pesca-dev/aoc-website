use cfg_if::cfg_if;

use leptos::*;

use crate::functions::{
    Login, LoginResult, Logout, Register, RegistrationResult, ResendVerificationMail,
    VerificationResult, Verify,
};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::{hooks::{use_identity, use_user}};
}
}

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<Login, Result<LoginResult, ServerFnError>>,
    pub register: Action<Register, Result<RegistrationResult, ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub verify: Action<Verify, Result<VerificationResult, ServerFnError>>,
    pub resend_verification_email: Action<ResendVerificationMail, Result<(), ServerFnError>>,
    pub user: Resource<(usize, usize, usize), Result<Option<String>, ServerFnError>>,
}

impl AuthContext {
    #[tracing::instrument(level = "trace")]
    fn new() -> Self {
        let login = create_server_action::<Login>();
        let logout = create_server_action::<Logout>();
        let register = create_server_action::<Register>();
        let verify = create_server_action::<Verify>();
        let resend_verification_email = create_server_action::<ResendVerificationMail>();

        let user = create_resource(
            move || {
                (
                    login.version().get(),
                    logout.version().get(),
                    register.version().get(),
                )
            },
            move |_| get_user_id(),
        );

        AuthContext {
            login,
            logout,
            register,
            verify,
            resend_verification_email,
            user,
        }
    }
}

#[tracing::instrument(level = "trace")]
#[server]
async fn get_user_id() -> Result<Option<String>, ServerFnError> {
    match use_user().await {
        Some(user) => {
            return Ok(Some(user.username));
        }
        None => {
            let identity = use_identity()?;
            identity.logout();
            return Err(ServerFnError::ServerError("Inactive session!".to_string()));
        }
    }
}

/// Provide an AuthContext for use in child components.
#[component]
pub fn AuthContextProvider(children: Children) -> impl IntoView {
    provide_context(AuthContext::new());

    children()
}
