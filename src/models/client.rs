//! Client mappings.

use rocket::{
    futures::stream::SplitSink,
    serde::{Deserialize, Serialize},
    tokio::{net::TcpStream, sync::Mutex},
};
use std::{net::SocketAddr, sync::Arc};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

/// A struct representing a gateway client.
pub struct Client {
    // `addr` mainly exists so that we can remove the clients from the connected gateway peers later.
    pub addr: SocketAddr,
    pub ws_sink: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}

pub type Clients = Arc<Mutex<Vec<Client>>>;

/// A struct representing a message a client sent via the rest api.
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientMessage {
    author: String,
    content: String,
}
