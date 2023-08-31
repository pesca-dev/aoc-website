mod auth;

use cfg_if::cfg_if;

pub use self::auth::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    mod database;
    mod identity;
    mod user;

    pub use self::database::*;
    pub use self::identity::*;
    pub use self::user::*;
}
}
