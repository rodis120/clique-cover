#[allow(unused_imports)]

use bytes::Bytes;
// use futures_util::{sink::SinkExt, stream::StreamExt};
use futures::future;
use futures::{SinkExt, StreamExt, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, Full, StreamBody, Empty};
use hyper::{Request, Response, StatusCode};
use hyper::body::{Body, Incoming, Frame};
use hyper::header;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade;
use hyper_util::rt::tokio::TokioIo;
use hyper_tungstenite::{WebSocketStream, HyperWebsocket};
use hyper_tungstenite::tungstenite::protocol::Message;
use serde_json;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_util::io::ReaderStream;

use shared::{Result, Error, MyMsg, RunParams, GraphDist};

use crate::types::{SharedState, Database};

async fn handle_websocket(
    ws: HyperWebsocket,
    state: WebsiteHandlerState,
) -> Result<()> {
    println!("website: handle_websocket: initiated");
    let mut ws = ws.await?;
    let (mut ws_write, mut ws_read) = ws.split();
    println!("website: handle_websocket: split");
    
    let shared_state = state.shared_state;
    
    let handles: Vec<tokio::task::JoinHandle<()>> = vec![
        {
            let shared_state = shared_state.clone();
            tokio::spawn(async move {
                println!("website: handle_websocket: started write task");
                let mut rx_at_website = state.rx_at_website;

                {
                    // FOR TESTING PURPOSES, REMOVE IT LATER
                    let msg = MyMsg::RequestRestart(
                        "xdddd".to_string(),
                        GraphDist::default(),
                        vec![2, 1, 3, 7],
                    );
                    if let Ok(serialized) = serde_json::to_string(&msg) {
                        let _ = ws_write.send(Message::Text(serialized)).await;
                    }
                    // -------------------------------------
                    
                    let graph_dist = shared_state.graph_dist.clone();
                    let msg = MyMsg::GraphDist(graph_dist);
                    if let Ok(serialized) = serde_json::to_string(&msg) {
                        let _ = ws_write.send(Message::Text(serialized)).await;
                    }

                    let algorithms = shared_state.algorithms.read().await.clone();
                    let msg = MyMsg::AlgoList(algorithms);
                    if let Ok(serialized) = serde_json::to_string(&msg) {
                        let _ = ws_write.send(Message::Text(serialized)).await;
                    }

                    let in_use = shared_state.algos_in_use.read().await.clone();
                    let msg = MyMsg::AlgosInUse(in_use);
                    if let Ok(serialized) = serde_json::to_string(&msg) {
                        let _ = ws_write.send(Message::Text(serialized)).await;
                    }

                    let graph_order = shared_state.database.get_graph_order().await;
                    for id in graph_order.into_iter() {
                        if let Some(graph) = shared_state.database.get_graph(id).await {
                            let msg = MyMsg::Graph(
                                shared_state.session_id.load(Ordering::Acquire),
                                id,
                                graph,
                            );
                            if let Ok(serialized) = serde_json::to_string(&msg) {
                                let _ = ws_write.send(Message::Text(serialized)).await;
                            }
                        }
                    }

                    let solution_order = shared_state.database.get_solution_order().await;
                    for id in solution_order.into_iter() {
                        let (algo_id, graph_id) = shared_state.database.get_algo_and_graph_id(id);
                        if let Some(solution) = shared_state
                            .database
                            .get_solution(algo_id, graph_id)
                            .await {
                                let msg = MyMsg::Solution(
                                    shared_state.session_id.load(Ordering::Acquire),
                                    algo_id,
                                    graph_id,
                                    solution,
                                );
                                if let Ok(serialized) = serde_json::to_string(&msg) {
                                    let _ = ws_write.send(Message::Text(serialized)).await;
                                }
                        }
                    }
                }

                loop {
                    sleep(Duration::from_secs(10)).await;
                }
                println!("website: handle_websocket: finished write task");
            })
        },
        {
            let shared_state = shared_state.clone();
            tokio::spawn(async move {
                println!("website: handle_websocket: started read task");
                let tx_to_algonet = state.tx_to_algonet;
                let tx_to_grafnet = state.tx_to_grafnet;
                loop {
                    if let Some(Ok(Message::Text(contents))) = ws_read.next().await {
                        println!("received from websocket: {}", contents);
                        match serde_json::from_str::<MyMsg>(&contents) {
                            Ok(msg) => {
                                println!("website: new message: {:?}", msg);
                                match msg {
                                    MyMsg::RequestRestart(password, graph_dist, algos_in_use) => {
                                        let shared_state = shared_state.clone();
                                        let tx_to_grafnet = tx_to_grafnet.clone();
                                        tokio::spawn(
                                            new_session(
                                                password,
                                                graph_dist,
                                                algos_in_use,
                                                tx_to_grafnet,
                                                shared_state,
                                            )
                                        );
                                    },
                                    _ => {},
                                };
                            },
                            Err(e) => {
                                eprintln!("website: failed to parse JSON: {}", e);
                            },
                        }
                    };
                }
                println!("website: handle_websocket: finished read task");
            })
        },
    ];

    let (_, _, tasks) = future::select_all(handles).await;
    
    println!("website: handle_websocket: aborting");
    
    for handle in tasks {
        handle.abort();
    }

    Ok(())
}

async fn new_session(
    password: String,
    graph_dist: GraphDist,
    algos_in_use: Vec<u16>,
    tx_to_grafnet: mpsc::Sender<MyMsg>,
    shared_state: Arc<SharedState>,
) {
    println!("website: starting new session:");
    println!("----graph_dist: {graph_dist:?}");
    println!("----algos_in_use: {algos_in_use:?}");

    let old_session_id = shared_state.session_id.fetch_add(1, Ordering::Relaxed);
    let new_session_id = shared_state.session_id.load(Ordering::Relaxed);
    println!("----old session_id: {}", old_session_id);
    println!("----new session_id: {}", new_session_id);

    {
        let mut guard = shared_state.algos_in_use.write().await;
        *guard = algos_in_use;
    }

    let grafnet_msg = MyMsg::GraphDist2Generate(new_session_id, graph_dist);

    if let Err(e) = tx_to_grafnet.send(grafnet_msg).await {
        eprintln!("website: failed to send GraphDist2Generate");
    }
}

async fn serve_file(path: &Path) -> Result<Response<BoxBody<Bytes, Infallible>>> {
    let file = File::open(path).await;
    if file.is_err() {
        eprintln!("unable to open file");
        return not_found();
    }
    let file = file.unwrap();

    let reader_stream = ReaderStream::new(file);
    let reader_stream = reader_stream.map_err(|_| {
        unreachable!("one error in the reader stream \
            and the entire app goes down, \
            but unfortunately i dont have the time \
            to come up with a better solution");
    });
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));
    let boxed_body = BodyExt::boxed(stream_body);
    
    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .as_ref()
        .to_string();

    let res = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", mime_type)
        .body(boxed_body)
        .expect("failed to produce a response");
    Ok(res)
}

async fn handle_request(
    mut req: Request<Incoming>,
    public_dir: Arc<PathBuf>,
    state: WebsiteHandlerState,
) -> Result<Response<BoxBody<Bytes, Infallible>>> {
    let path = req.uri().path();
    println!("request for {:?} reached handle_request", path);
    match path {
        "/" => {
            let path = public_dir.clone().join("index.html");
            match serve_file(path.as_path()).await {
                Ok(res) => Ok(res),
                Err(_) => internal_server_error(),
            }
        },
        "/ws" => {
            if !hyper_tungstenite::is_upgrade_request(&req) {
                eprintln!("bad upgrade request");
                return bad_request();
            }
            let (response, ws) = hyper_tungstenite::upgrade(&mut req, None)?;
            tokio::spawn(async move {
                if let Err(e) = handle_websocket(
                    ws,
                    state,
                ).await {
                    eprintln!("failed to handle websocket");
                }
            });

            let boxed = response.map(|full| full.boxed());
            Ok(boxed)
        },
        _ => {
            if path.contains("..") {
                return bad_request();
            }
            let path = path.trim_start_matches('/');
            let path = public_dir.clone().join(path);
            match serve_file(path.as_path()).await {
                Ok(res) => Ok(res),
                Err(_) => internal_server_error(),
            }
        },
    }
}

pub async fn handle_website(
    socket_addr: SocketAddr,
    public_dir: PathBuf,
    rx_at_website: broadcast::Receiver<MyMsg>,
    tx_to_algonet: broadcast::Sender<MyMsg>,
    tx_to_grafnet: mpsc::Sender<MyMsg>,
    shared_state: Arc<SharedState>,
) -> Result<()> {
    println!("website_handler: initiated");

    let public_dir = Arc::new(public_dir);
    let listener = TcpListener::bind(socket_addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        println!("website_handler: new stream accepted");
        let io = TokioIo::new(stream);

        let public_dir = public_dir.clone();
        let state = WebsiteHandlerState {
            rx_at_website: rx_at_website.resubscribe(),
            tx_to_algonet: tx_to_algonet.clone(),
            tx_to_grafnet: tx_to_grafnet.clone(),
            shared_state: shared_state.clone(),
        };

        let service = service_fn(move |req| {
            println!("request for {:?} reached service_fn", req.uri().path());
            handle_request(
                req,
                public_dir.clone(),
                state.clone(),
            )
        });
        let conn = http1::Builder::new()
            .serve_connection(io, service)
            .with_upgrades();

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("website_handler: could not serve connvection: {}", e);
            }
        });
    }
}

struct WebsiteHandlerState {
    rx_at_website: broadcast::Receiver<MyMsg>,
    tx_to_algonet: broadcast::Sender<MyMsg>,
    tx_to_grafnet: mpsc::Sender<MyMsg>,
    shared_state: Arc<SharedState>,
}

impl Clone for WebsiteHandlerState {
    fn clone(&self) -> Self {
        Self {
            rx_at_website: self.rx_at_website.resubscribe(),
            tx_to_algonet: self.tx_to_algonet.clone(),
            tx_to_grafnet: self.tx_to_grafnet.clone(),
            shared_state: self.shared_state.clone(),
        }
    }
}

fn not_found() -> Result<Response<BoxBody<Bytes, Infallible>>> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from_static(b"not found")).boxed())
        .unwrap())
}

fn bad_request() -> Result<Response<BoxBody<Bytes, Infallible>>> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(Bytes::from_static(b"bad request")).boxed())
        .unwrap())
}

fn internal_server_error() -> Result<Response<BoxBody<Bytes, Infallible>>> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::new(Bytes::from_static(b"internal server error")).boxed())
        .unwrap())
}

fn switching_protocols() -> Result<Response<BoxBody<Bytes, Infallible>>> {
    Ok(Response::builder()
        .status(StatusCode::SWITCHING_PROTOCOLS)
        .header("Upgrade", "websocket")
        .header("Connection", "Upgrade")
        .header("Sec-WebSocket-Accept", "")
        .body(Empty::new().boxed())
        .unwrap())
}

