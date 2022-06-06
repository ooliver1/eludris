//! The Eludris gateway.

use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    tokio::{
        net::{TcpListener, TcpStream},
        sync::Mutex,
        task,
        time::sleep,
        select,
    },
};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use crate::{
    models::client::{Client, Clients},
    utils::now_timestamp,
};

async fn check_connection(
    last_ping: Arc<Mutex<u32>>,
    stream: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
) {
    loop {
        if (*last_ping.lock().await + 20) < now_timestamp() {
            println!("Over.");
            stream
                .lock()
                .await
                .close()
                .await
                .expect("Couldn't close ws.");
        }
        sleep(Duration::from_secs(20)).await;
    }
}

/// A function that handles one client connecting and disconnecting.
async fn handle_connection(addr: SocketAddr, stream: TcpStream, clients: Clients) {
    let socket = accept_async(stream)
        .await
        .expect("Couldn't accept the socket stream.");

    let (outgoing, mut incoming) = socket.split();
    let outgoing = Arc::new(Mutex::new(outgoing));

    let last_ping = Arc::new(Mutex::new(now_timestamp()));

    {
        let mut clients = clients.lock().await;
        clients.push(Client {
            addr,
            ws_sink: outgoing.clone(),
        });
    }

    let handle_incoming = async {
        while let Some(msg) = incoming.next().await {
            log::debug!("{:#?}", msg);
            match msg {
                Ok(data) => match data {
                    Message::Ping(x) => {
                        *last_ping.lock().await = now_timestamp();
                        outgoing
                            .lock()
                            .await
                            .send(Message::Pong(x))
                            .await
                            .expect("Couldn't send pong");
                    }
                    _ => {}
                },
                Err(_) => break,
            }
        }
    };

    select!{
        _ = check_connection(last_ping.clone(), outgoing.clone()) => {},
        _ = handle_incoming => {},
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
