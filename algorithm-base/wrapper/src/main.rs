use std::env;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use shared::{MyMsg, Graph, Solution, Result};

const URL: &str = "ws://127.0.0.1:8080";


#[tokio::main]
async fn main() -> Result<()> {
    let algo_name = env::var("ALGO_NAME")?;
    println!("hello from algorithm-base: {}", algo_name);
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5));
    }

    Ok(())
}

