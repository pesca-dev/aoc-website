use std::env;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn use_database(db: impl ToString) -> Surreal<Client> {
    let connection =
        Surreal::new::<Ws>(env::var("SURREAL_URL").expect("no surreal url db user given"))
            .await
            .expect("could not connect to database");

    connection
        .signin(Root {
            // TODO: these should not be hardcoded but rather extracted from environment
            username: &env::var("SURREAL_USER").expect("no surreal db user given"),
            password: &env::var("SURREAL_PW").expect("no surreal pw given"),
        })
        .await
        .expect("could not login to database");

    connection
        .use_ns("aoc-website")
        .use_db(db.to_string())
        .await
        .expect("could not switch to correct namespace");

    connection
}
