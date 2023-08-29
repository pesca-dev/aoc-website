use cfg_if::cfg_if;

use leptos::*;

use crate::functions::{Login, Logout, Register, RegistrationResult};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::{hooks::use_identity, model::Session};
}
}

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<Login, Result<(), ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub register: Action<Register, Result<RegistrationResult, ServerFnError>>,
    pub user: Resource<(usize, usize, usize), Result<Option<String>, ServerFnError>>,
}

impl AuthContext {
    fn new(cx: Scope) -> Self {
        let login = create_server_action::<Login>(cx);
        let logout = create_server_action::<Logout>(cx);
        let register = create_server_action::<Register>(cx);

        let user = create_resource(
            cx,
            move || {
                (
                    login.version().get(),
                    logout.version().get(),
                    register.version().get(),
                )
            },
            move |_| get_user_id(cx),
        );

        AuthContext {
            login,
            logout,
            register,
            user,
        }
    }
}

#[server(GetUserId, "/api")]
async fn get_user_id(cx: Scope) -> Result<Option<String>, ServerFnError> {
    let identity = use_identity(cx)?;

    let session_id = identity
        .id()
        .map_err(|_| ServerFnError::ServerError("User Not Found!".to_string()))?;

    match Session::find_user_via_session(&session_id).await {
        Some(user) => {
            return Ok(Some(user.username));
        }
        None => {
            identity.logout();
            return Err(ServerFnError::ServerError("Inactive session!".to_string()));
        }
    }
}

/// Provide an AuthContext for use in child components.
#[component]
pub fn AuthContextProvider(cx: Scope, children: Children) -> impl IntoView {
    provide_context(cx, AuthContext::new(cx));

    children(cx)
}
