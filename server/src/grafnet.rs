use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use bytes::Bytes;
use http_body_util::{BodyExt, Empty};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use shared::{Result, MyMsg, Graph, GraphDist};

use crate::types::{SharedState, Database};

#[derive(Debug, Deserialize)]
struct Response {
    degree: u16,
    nodes: u16,
    encodedGraph: String,
    size: usize,
}

fn parse_response(response: String) -> Result<Response> {
    let output: Response = serde_json::from_str(&response)
        .map_err(|_| "grafnet: failed to parse response json".to_string())?;
    Ok(output)
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
    let client = reqwest::Client::new();
    
    let response = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    let graph_string = parse_response(response)?.encodedGraph;

    Ok(Graph { inner: graph_string })
}

pub async fn handle_grafnet(
    graph_gen_path: String,
    mut rx_at_grafnet: mpsc::Receiver<MyMsg>,
    tx_to_algonet: broadcast::Sender<MyMsg>,
    tx_to_website: broadcast::Sender<MyMsg>,
    shared_state: Arc<SharedState>,
) {
    println!("grafnet: initiated");

    while let Some(MyMsg::GraphDist2Generate(session_id, graph_dist))
        = rx_at_grafnet.recv().await
    {
        println!("grafnet: received: {:?}", graph_dist);
        println!("grafnet: sending requests to {}", graph_gen_path);

        let (_, h_size, v_size, _) = shared_state
            .database.read().await
            .get_primitive_fields();

        let n_graphs = h_size as u32 * v_size as u32;
        let GraphDist { 
            n_nodes_min: nodes_min,
            n_nodes_step: nodes_step,
            ..
        } = graph_dist;

        let mut handles: Vec<_> = Vec::new();

        for graph_i in 0..n_graphs {
            let graph_gen_path = graph_gen_path.clone();
            let shared_state = shared_state.clone();
            let tx_to_algonet = tx_to_algonet.clone();
            let tx_to_website = tx_to_website.clone();
            let n_nodes = nodes_min + nodes_step * (graph_i as u16 / h_size);
            let session_id = shared_state.session_id.load(Ordering::Relaxed);

            handles.push(tokio::spawn(async move {
                let shared_state = shared_state.clone();
                let tx_to_algonet = tx_to_algonet.clone();
                let tx_to_website = tx_to_website.clone();
                let graph = request_graph(
                    n_nodes,
                    graph_dist.node_density,
                    graph_gen_path.clone(),
                ).await;
                match graph {
                    Ok(graph) => {
                        println!("grafnet: id={graph_i}, nodes={n_nodes}");

                        shared_state
                            .database.read().await
                            .insert_graph(graph, graph_i).await;

                        
                        let msg = MyMsg::GraphReady(session_id, graph_i);
                        if let Err(e) = tx_to_algonet.send(msg.clone()) {
                            eprintln!("grafnet: failed to send a message to algonet");
                        }
                        if let Err(e) = tx_to_website.send(msg.clone()) {
                            eprintln!("grafnet: failed to send a message to website");
                        }

                    },
                    Err(e) => {
                        eprintln!("grafnet: graph request failed: {}", e);
                    },
                };
            }));
        }
        futures::future::join_all(handles).await;
        println!("grafnet: graphs in database:");
        shared_state.database.read().await.print().await;
    }
}

