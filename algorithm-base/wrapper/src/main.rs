use futures::future;
use futures::{SinkExt, StreamExt};
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use shared::{MyMsg, Graph, Solution, Result};

const URL: &str = "ws://server:3001";

#[tokio::main]
async fn main() -> Result<()> {
    let algo_name = env::var("ALGO_NAME")?;
    println!("hello from algorithm-base: {}", algo_name);

    // Establish a WebSocket connection
    let ((ws_stream, _)) = connect_async(URL).await?;
    let (mut ws_write, mut ws_read) = ws_stream.split();

    // Greet the server (and tell it your name)
    {
        let greeting = MyMsg::Greet(algo_name.clone());
        let serialized = bincode::serialize(&greeting)
            .map_err(|_| "failed to serialize greeting")?;
        let _ = ws_write
            .send(Message::Binary(serialized))
            .await
            .map_err(|_| "failed to send greeting")?;
    }

    // For every message (graph) received from the server...
    while let ws_data = ws_read.next().await {
        match ws_data {
            None => {
                eprintln!("websocket connection has closed");
            },
            Some(Err(e)) => {
                eprintln!("websocket failed to read: {}", e.to_string());
            },
            Some(Ok(Message::Binary(contents))) => {
                tokio::spawn(handle_graph(contents));
            },
            _ => {
                eprintln!("a message received from websocket, but the format is non-binary");
            },
        }
    }

    Ok(())
}

async fn handle_graph(message_bytes: Vec<u8>) {
    if let Ok(msg) = bincode::deserialize::<MyMsg>(&message_bytes) {
        if let MyMsg::Graph(session_id, graph_id, graph) = msg {
            println!("received graph#{graph_id}");
            // TODO: spawn a child process to handle the graph
        }
    }
}
