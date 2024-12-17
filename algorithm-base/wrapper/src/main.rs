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

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5));
    }

    Ok(())
}

