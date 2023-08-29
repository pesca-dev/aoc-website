use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    mod user;
    mod session;

    pub use self::user::*;
    pub use self::session::*;
}
}
