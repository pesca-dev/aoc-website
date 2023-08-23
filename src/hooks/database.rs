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
        username: "root",
        password: "root",
    })
    .await
    .expect("could not login to database");

    db.use_ns("test")
        .use_db(ns.to_string())
        .await
        .expect("could not switch to correct namespace");

    db
}
