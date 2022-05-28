#[macro_use]
extern crate rocket;

use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    serde::{json::Json, Deserialize, Serialize},
    tokio::{
        net::{TcpListener, TcpStream},
        sync::Mutex,
        task,
    },
    State,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde_json::to_string;
use sqlx::MySqlPool;
use std::{collections::HashMap, env, net::SocketAddr, str::FromStr, sync::Arc};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

// A HashMap of addresses as keys and sinks as values in a Mutex that's in an Arc.
type Peers = Arc<Mutex<HashMap<SocketAddr, SplitSink<WebSocketStream<TcpStream>, Message>>>>;

// type Database = Arc<Mutex<MySqlPool>>;

// The only struct here, used in both json payloads and ws events.
#[derive(Debug, Deserialize, Serialize)]
struct ClientMessage {
    author: String,
    content: String,
}

// An example route that expects a payload of `ClientMessage` and broadcasts it to everyone connected to the websocket.
#[post("/", format = "json", data = "<message>")]
async fn index(state: &State<Peers>, message: Json<ClientMessage>) {
    let message = to_string(&message.into_inner()).expect("Couldn't conver the message to json.");
    let mut peers = state.lock().await;
    for (_, peer) in peers.iter_mut() {
        peer.send(Message::Text(message.clone()))
            .await
            .expect("Couldn't send the message.");
    }
}

// A function that handles one peer connecting and disconnecting.
async fn handle_connection(addr: SocketAddr, stream: TcpStream, peers: Peers) {
    let socket = accept_async(stream)
        .await
        .expect("Couldn't accept the socket stream.");

    let (outgoing, mut incoming) = socket.split();

    {
        let mut peers = peers.lock().await;
        peers.insert(addr, outgoing);
    }

    while let Some(msg) = incoming.next().await {
        match msg {
            Ok(_) => {}
            Err(_) => break,
        }
    }

    log::info!("Someone disconnected");

    {
        let mut peers = peers.lock().await;
        peers.remove(&addr);
    }
}

// A function that starts the websocket and uses `handle_connection` for every peer.
async fn handle_ws(state: Peers) {
    let ws_address = env::var("WS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:5000".to_string());
    let socket = TcpListener::bind(&ws_address)
        .await
        .unwrap_or_else(|_| panic!("Couldn't start a websocket on {}", ws_address));
    log::info!("ws server started");

    while let Ok((stream, addr)) = socket.accept().await {
        log::info!("New connection");
        let clients = state.clone();
        task::spawn(handle_connection(addr, stream, clients));
    }
}

#[rocket::main]
async fn main() {
    // Starting logger.
    env_logger::init();

    // A HashMap for storing peers.
    let state = Arc::new(Mutex::new(HashMap::new()));

    // Establishing a DB connection.

    let pool = Arc::new(Mutex::new(
        MySqlPool::connect(
            &env::var("DATABASE_URL").expect("\"DATABASE_URL\" enviroment variable not found."),
        )
        .await
        .expect("Couldn't establish a connection with the database."),
    ));

    // Cors stuff.
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec!["GET", "POST"]
            .iter()
            .map(|m| FromStr::from_str(m).expect("Couldn't generate cors methods."))
            .collect(),
        ..Default::default()
    }
    .to_cors()
    .expect("Couldn't create CorsOptions struct.");

    // The websocket task.
    task::spawn(handle_ws(state.clone()));

    // The rest API.
    #![allow(clippy::all)] rocket::build()
        .mount("/", routes![index])
        .manage(state)
        .manage(pool)
        .attach(cors)
        .launch()
        .await
        .expect("Couldn't launch rocket rest api."); // Ah yes, rocket::Rocket has a must_use lint!
}
