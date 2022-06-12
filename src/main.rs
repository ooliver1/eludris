mod gateway;
mod models;
mod rest;
mod utils;

#[macro_use]
extern crate rocket;

use redis::Client;
use rocket::tokio::{sync::Mutex, task};
use sqlx::MySqlPool;
use std::{env, sync::Arc};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Starting logger.
    env_logger::init();

    // A HashMap for storing peers.
    let clients = Arc::new(Mutex::new(Vec::new()));

    // Establishing a DB connection.
    let db = Arc::new(Mutex::new(
        MySqlPool::connect(
            &env::var("DATABASE_URL").expect("\"DATABASE_URL\" enviroment variable not found."),
        )
        .await
        .expect("Couldn't establish a connection with the database."),
    ));

    // Establishing a Redis connection.
    let client =
        Client::open(env::var("REDIS_URL").expect("\"REDIS_URL\" enviroment variable not found"))
            .expect("Couldn't connect to Redis at the specified URL.");
    // TODO: do something with this.
    let _con = Arc::new(Mutex::new(
        client
            .get_async_connection()
            .await
            .expect("Couldn't obtain Redis connection"),
    ));

    // Start a task for the gateway.
    task::spawn(gateway::start(clients.clone()));

    // Start the rest api.
    rest::start(clients, db).await?;

    Ok(())
}
