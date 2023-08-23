use cfg_if::cfg_if;

use leptos::*;

use crate::functions::{Login, Logout, Register};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use crate::hooks::use_identity;
}
}

#[derive(Clone)]
pub struct AuthContext {
    pub login: Action<Login, Result<(), ServerFnError>>,
    pub logout: Action<Logout, Result<(), ServerFnError>>,
    pub register: Action<Register, Result<(), ServerFnError>>,
    pub user: Resource<(usize, usize), Result<String, ServerFnError>>,
}

impl AuthContext {
    fn new(cx: Scope) -> Self {
        let login = create_server_action::<Login>(cx);
        let logout = create_server_action::<Logout>(cx);
        let register = create_server_action::<Register>(cx);

        let user = create_resource(
            cx,
            move || (login.version().get(), logout.version().get()),
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
async fn get_user_id(cx: Scope) -> Result<String, ServerFnError> {
    let identity = use_identity(cx)?;

    let id = identity
        .id()
        .map_err(|_| ServerFnError::ServerError("User Not Found!".to_string()))?;

    Ok(id)
}

/// Provide an AuthContext for use in child components.
#[component]
pub fn AuthContextProvider(cx: Scope, children: Children) -> impl IntoView {
    provide_context(cx, AuthContext::new(cx));

    children(cx)
}
