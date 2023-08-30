use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    mod user;
    mod session;
    mod logged_in;

    pub use self::user::*;
    pub use self::session::*;
    pub use self::logged_in::*;
}
}
