use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    pub mod password;

}
}
