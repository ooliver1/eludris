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
type Database = Arc<Mutex<MySqlPool>>;

// The only struct here, used in both json payloads and ws events.
#[derive(Debug, Deserialize, Serialize, Clone)]
struct ClientMessage {
    author: String,
    content: String,
}

// An example route that expects a payload of `ClientMessage` and broadcasts it to everyone connected to the websocket.
#[post("/", format = "json", data = "<message>")]
async fn index(state: &State<Peers>, message: Json<ClientMessage>) {
    let message = message.into_inner();
    let mut peers = state.lock().await;
    for (_, peer) in peers.iter_mut() {
        peer.send(Message::Text(to_string(&message).unwrap()))
            .await
            .unwrap();
    }
}

// Showcasing a route communicating with the database.
#[get("/test")]
async fn test(state: &State<Database>) -> String {
    let db = state.lock().await;
    let res = sqlx::query!("SELECT * FROM users")
        .fetch_all(&*db) // mmlol
        .await
        .unwrap();

    // Probably shitty code
    let mut out = Vec::new();
    for r in res {
        if let Some(test) = r.username {
            out.push(test);
        }
    }
    out.join("\n")
}

// A function that handles one peer connecting and disconnecting.
async fn handle_connection(addr: SocketAddr, stream: TcpStream, peers: Peers) {
    let socket = accept_async(stream).await.unwrap();

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

    println!("Someone disconnected");

    {
        let mut peers = peers.lock().await;
        peers.remove(&addr);
    }
}

// A function that starts the websocket and uses `handle_connection` for every peer.
async fn handle_ws(state: Peers) {
    let socket = TcpListener::bind("0.0.0.0:8001").await.unwrap();
    println!("ws server started");

    while let Ok((stream, addr)) = socket.accept().await {
        println!("New connection");
        let clients = state.clone();
        task::spawn(handle_connection(addr, stream, clients));
    }
}

#[rocket::main]
async fn main() {
    let state = Arc::new(Mutex::new(HashMap::new()));

    // Establishing a DB connection.
    let pool = Arc::new(Mutex::new(
        MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    ));

    // Cors stuff
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec!["GET", "POST"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect(),
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    // The websocket task.
    task::spawn(handle_ws(state.clone()));

    // The rest API.
    rocket::build()
        .mount("/", routes![index, test])
        .manage(state)
        .manage(pool)
        .attach(cors)
        .launch()
        .await
        .unwrap();
}
