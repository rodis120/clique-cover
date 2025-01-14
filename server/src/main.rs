use futures::future;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::lookup_host;
use tokio::sync::{broadcast, mpsc};

use shared::MyMsg;

mod types;
use types::{SharedState, Database};

mod website;
use website::handle_website;

mod algonet;
use algonet::handle_algonet;

mod grafnet;
use grafnet::handle_grafnet;

const HASH_SEED: u64 = 0xdb2137db;
const CHANNEL_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let public_dir = PathBuf::from("/app/public");

    let website_addr: SocketAddr = ([0, 0, 0, 0,], 3000).into();
    let algonet_addr: SocketAddr = ([0, 0, 0, 0,], 3001).into();
    let graph_gen_path: String = "http://graph-generator:8123/gen_graph".to_string();

    let (tx_to_algonet, mut rx_at_algonet) = broadcast::channel::<MyMsg>(CHANNEL_SIZE);
    let (tx_to_website, mut rx_at_website) = broadcast::channel::<MyMsg>(CHANNEL_SIZE);
    let (tx_to_grafnet, mut rx_at_grafnet) = mpsc::channel::<MyMsg>(CHANNEL_SIZE);

    let shared_state = Arc::new(SharedState::new());
    
    let handles: Vec<tokio::task::JoinHandle<()>> = vec![
        {
            let tx_to_algonet = tx_to_algonet.clone();
            let tx_to_grafnet = tx_to_grafnet.clone();
            let shared_state = shared_state.clone();
            tokio::spawn(async move {
                handle_website(
                    website_addr,
                    public_dir,
                    rx_at_website,
                    tx_to_algonet.clone(),
                    tx_to_grafnet.clone(),
                    shared_state.clone(),
                ).await;
            })
        },
        {
            let tx_to_website = tx_to_website.clone();
            let shared_state = shared_state.clone();
            tokio::spawn(async move {
                handle_algonet(
                    algonet_addr,
                    rx_at_algonet,
                    tx_to_website.clone(),
                    shared_state.clone(),
                ).await;
            })
        },
        {
            let tx_to_algonet = tx_to_algonet.clone();
            let tx_to_website = tx_to_website.clone();
            tokio::spawn(async move {
                handle_grafnet(
                    graph_gen_path,
                    rx_at_grafnet,
                    tx_to_algonet.clone(),
                    tx_to_website.clone(),
                ).await;
            })
        }
    ];

    let (_, _, tasks) = future::select_all(handles).await;

    println!("main: aborting");
    for handle in tasks {
        handle.abort();
    }

    Ok(())
}
