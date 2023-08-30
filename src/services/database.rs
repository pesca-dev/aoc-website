use std::{env, sync::OnceLock};

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub static DB: OnceLock<Surreal<Client>> = OnceLock::new();

pub async fn init_db() -> Result<(), surrealdb::Error> {
    let connection =
        Surreal::new::<Ws>(env::var("SURREAL_URL").expect("no surreal url db user given")).await?;

    connection
        .signin(Root {
            // TODO: these should not be hardcoded but rather extracted from environment
            username: &env::var("SURREAL_USER").expect("no surreal db user given"),
            password: &env::var("SURREAL_PW").expect("no surreal pw given"),
        })
        .await?;

    DB.get_or_init(move || connection);

    Ok(())
}
