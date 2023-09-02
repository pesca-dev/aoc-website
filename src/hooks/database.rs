use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::services::database::DB;

const NS_NAME: &str = "aoc_website";

const DB_NAME: &str = "aoc_website";

#[tracing::instrument(level = "trace")]
pub async fn use_database<'a>() -> &'a Surreal<Client> {
    let connection = DB
        .get()
        .expect("call to use_database before calling database::init");

    connection
        .use_ns(NS_NAME)
        .use_db(DB_NAME)
        .await
        .expect("could not switch to correct namespace");

    connection
}
