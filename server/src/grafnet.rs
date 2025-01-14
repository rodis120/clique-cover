use std::net::SocketAddr;
use std::sync::Arc;
use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use shared::{Result, MyMsg, Graph};

use crate::types::{SharedState, Database};

#[derive(Debug, Deserialize)]
struct Response {
    degree: u16,
    nodes: u16,
    encodedGraph: String,
    size: usize,
}

fn parse_response(response: String) -> Result<Response> {
    let output: Response = serde_json::from_str(response)
        .expect("grafnet: failed to parse response json");
    output
}

async fn request_graph(
    n_nodes: u16,
    density: f64,
    url: String,
) -> Result<Graph> {
    // remember that degree must be even or n_nodes must be even
    let degree: u16 = {
        let degree = (n_nodes as f64 * density) as u16;
        if degree % 2 == 1 && n_nodes % 2 == 1 {
            degree - 1
        } else {
            degree
        }
    };

    let url = format!("{}?nodes={}&degree={}", url, n_nodes, degree);

    let response = ureq::get(&url)
        .call()
        .map_err(|e| format!("grafnet: request failed {}: {e}", url))?
        .into_string()
        .map_err(|_| format!("grafnet: failed to parse response from request {}", url))?;

    let graph_bytes = parse_response(response)?.encodedGraph.into_bytes();

    Ok(Graph { inner: graph_bytes })
}

pub async fn handle_grafnet(
    graph_gen_path: String,
    mut rx_at_grafnet: mpsc::Receiver<MyMsg>,
    tx_to_algonet: broadcast::Sender<MyMsg>,
    tx_to_website: broadcast::Sender<MyMsg>,
) {
    println!("grafnet: initiated");

    while let Some(MyMsg::GraphDist2Generate(session_id, graph_dist))
        = rx_at_grafnet.recv().await
    {
        println!("grafnet: received: {:?}", graph_dist);

        println!("grafnet: sending requests to {}", graph_gen_path);

        let mut n_nodes = graph_dist.n_nodes_min;
        while n_nodes <= graph_dist.n_nodes_max {
            let graph_gen_path = graph_gen_path.clone();
            tokio::spawn(async move {
                let graph = request_graph(
                    n_nodes,
                    graph_dist.node_density,
                    graph_gen_path.clone(),
                ).await;
                match graph {
                    Ok(graph) => {
                        println!("grafnet: graph of {n_nodes} nodes received successfully");
                        // TODO
                    },
                    Err(e) => {
                        eprintln!("grafnet: graph request failed: {}", e);
                    },
                };
            });

            n_nodes += graph_dist.n_nodes_step;
        }
    }
}

