//! The Eludris gateway.

use rocket::{
    futures::StreamExt,
    tokio::{
        net::{TcpListener, TcpStream},
        task,
    },
};
use std::{env, net::SocketAddr};
use tokio_tungstenite::accept_async;

use crate::{
    models::client::{Client, Clients},
    utils::now_timestamp,
};

/// A function that handles one client connecting and disconnecting.
async fn handle_connection(addr: SocketAddr, stream: TcpStream, clients: Clients) {
    let socket = accept_async(stream)
        .await
        .expect("Couldn't accept the socket stream.");

    let (outgoing, mut incoming) = socket.split();

    {
        let mut clients = clients.lock().await;
        clients.push(Client {
            addr,
            ws_sink: outgoing,
            last_ping: now_timestamp(),
        });
    }

    while let Some(msg) = incoming.next().await {
        match msg {
            Ok(data) => {
                println!("{:#?}", data)
            }
            Err(_) => break,
        }
    }

    log::info!("Someone disconnected");

    {
        let mut clients = clients.lock().await;
        let idex = clients
            .iter()
            .position(|c| c.addr == addr)
            .expect("Client not found");
        // We use `swap_remove` instead of `remove` here because it has O(1) complexity and we
        // don't really care about the orders of the clients here.
        clients.swap_remove(idex);
    }
}

/// A function that starts & handles the gatway.
pub async fn start(clients: Clients) {
    let ws_address = env::var("WS_ADDRESS").unwrap_or_else(|_| "0.0.0.0:5000".to_string());
    let socket = TcpListener::bind(&ws_address)
        .await
        .unwrap_or_else(|_| panic!("Couldn't start a websocket on {}", ws_address));
    log::info!("ws server started");

    while let Ok((stream, addr)) = socket.accept().await {
        log::info!("New connection");
        task::spawn(handle_connection(addr, stream, clients.clone()));
    }
}
