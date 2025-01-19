use futures::future;
use futures::{SinkExt, StreamExt};
use std::env;
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use shared::{MyMsg, Graph, Solution, Result};

const URL: &str = "ws://server:3001";
const SOLUTION_TEST_CMD: &str = "/app/solution-test";

#[tokio::main]
async fn main() -> Result<()> {
    let algo_name = env::var("ALGO_NAME")
        .map_err(|_| "ALGO_NAME not set")?;
    println!("hello from algorithm-base: {}", algo_name);

    let algo_run_cmd = env::var("ALGO_RUN_CMD")
        .map_err(|_| "ALGO_RUN_CMD not set")?;

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
                tokio::spawn(handle_graph(contents, algo_run_cmd.clone()));
            },
            _ => {
                eprintln!("a message received from websocket, but the format is non-binary");
            },
        }
    }

    Ok(())
}

async fn handle_graph(message_bytes: Vec<u8>, run_cmd: String) -> Result<()> {
    if let Ok(msg) = bincode::deserialize::<MyMsg>(&message_bytes) {
        if let MyMsg::Graph(session_id, graph_id, graph) = msg {
            println!("received graph#{graph_id}");
            
            let graph_contents = graph.inner;
            let run_cmd = format!("{} {} {} {}",
                "perf stat -e cycles",
                run_cmd,
                graph_contents.clone(),
                "2>&1 | awk \'NR == 1 {print $0} $2 == \"cycles\" {print $1}\'",
            );

            let output = Command::new("sh")
                .arg("-c")
                .arg(run_cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await?;
            
            let lines = String::from_utf8(output.stdout)?
                .split('\n')
                .map(|s| String::from(s))
                .collect::<Vec<String>>();
            let mut lines_iter = lines.into_iter();

            let (solution, n_cycles) = 
                if let Some(solution) = lines_iter.next() { 
                    if let Some(n_cycles) = lines_iter.next() {
                        if let Ok(n_cycles) = n_cycles.parse::<u64>() {
                            Ok((solution, n_cycles))
                        } else {
                            Err("cmd_output: failed to parse n_cycles")
                        }
                    } else {
                        Err("cmd_output: 1 line provided, 2 required")
                    }
                } else {
                    Err("cmd_output: 0 lines provided, 2 required")
                }?;

            println!("graph#{graph_id}:contents:{graph_contents}");
            println!("graph#{graph_id}:n_cycles:{n_cycles}");
            println!("graph#{graph_id}:solution:{solution}");

            let solution_test_cmd = format!("{} {} {}",
                SOLUTION_TEST_CMD,
                graph_contents,
                solution,
            );
            
            let output = Command::new("sh")
                .arg("-c")
                .arg(solution_test_cmd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await?;

            let output = String::from_utf8(output.stdout)?;
            println!("graph#{graph_id}:");
            println!("---output: {output}");
        }
    }
    Ok(())
}
