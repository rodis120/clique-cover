use futures::future;
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt};
use tokio::net::{TcpStream, TcpListener};
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use twox_hash::XxHash64;
use shared::{MyMsg, Graph, Solution, Result};
use crate::types::SharedState;

pub async fn handle_algonet(
    socket_addr: SocketAddr,
    rx_at_algonet: broadcast::Receiver<MyMsg>,
    tx_to_website: broadcast::Sender<MyMsg>,
    shared_state: Arc<SharedState>,
) -> Result<()> {
    let listener = TcpListener::bind(socket_addr).await?;
    let state = AlgonetHandlerState {
        rx_at_algonet,
        tx_to_website,
        shared_state,
    };
    println!("algonet: initiated");

    loop {
        let (tcp_stream, _) = listener.accept().await?;
        let ws_stream = accept_async(tcp_stream).await?;
        let state = state.clone();
        tokio::spawn(async move {
            handle_algonet_ws(ws_stream, state).await;
        });
    }
}

struct AlgonetHandlerState {
    rx_at_algonet: broadcast::Receiver<MyMsg>,
    tx_to_website: broadcast::Sender<MyMsg>,
    shared_state: Arc<SharedState>,
}

impl Clone for AlgonetHandlerState {
    fn clone(&self) -> Self {
        Self {
            rx_at_algonet: self.rx_at_algonet.resubscribe(),
            tx_to_website: self.tx_to_website.clone(),
            shared_state: self.shared_state.clone(),
        }
    }
}

async fn handle_algonet_ws(
    ws_stream: WebSocketStream<TcpStream>,
    state: AlgonetHandlerState,
) {
    println!("algonet: handle_websocket: initiated");
    let (mut ws_write, mut ws_read) = ws_stream.split();

    let handles: Vec<tokio::task::JoinHandle<()>> = vec![
        {
            let shared_state = state.shared_state.clone();
            let tx_to_website = state.tx_to_website;
            tokio::spawn(async move {
                while let Some(Ok(Message::Binary(contents))) = ws_read.next().await {
                    let deserialized: std::result::Result<MyMsg, _> 
                        = bincode::deserialize(&contents);
                    if let Ok(MyMsg::Greet(name)) = deserialized {
                        println!("A new algorithm signed up: {}", name);
                    }
                }
            })
        },
    ];

    let (_, _, tasks) = future::select_all(handles).await;

    println!("algonet_handler: aborting");
    for handle in tasks {
        handle.abort();
    }
}

