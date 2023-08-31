use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    mod user;

    pub use self::user::*;
}
}
