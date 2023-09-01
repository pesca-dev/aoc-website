use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    pub mod database;
    pub mod mail;
    pub mod jwt;
}
}
