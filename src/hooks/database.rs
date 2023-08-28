use std::env;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn use_database(ns: impl ToString) -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000")
        .await
        .expect("could not connect to database");

    db.signin(Root {
        // TODO: these should not be hardcoded but rather extracted from environment
        username: &env::var("SURREAL_USER").expect("no surreal db user given"),
        password: &env::var("SURREAL_PW").expect("no surreal pw given"),
    })
    .await
    .expect("could not login to database");

    db.use_ns("aoc-website")
        .use_db(ns.to_string())
        .await
        .expect("could not switch to correct namespace");

    db
}
