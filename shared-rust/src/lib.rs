use serde::{Serialize, Deserialize};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MyMsg {
    Session(u8),

    AlgoList(Vec<(u16, String)>),
    AlgosInUse(Vec<u16>),
    Graph(u8, u32, Graph), // session, graph
    Solution(u8, u16, u32, Solution), // session, algo, graph
    GraphDist(GraphDist),

    Greet(String),

    SolutionReady(u8, u16, u32), // session, algo, graph
    SolutionProduced(u8, u32, Solution), // session, graph, Solution

    GraphReady(u8, u32),
    GraphProduced(u8, Graph),

    GraphDist2Generate(u8, GraphDist),
    Restart(Vec<(u16, u16)>), // general_algo_id, session_specific_algo_id
    RequestRestart(String, GraphDist, Vec<u16>),

}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Algo {
    pub id: u16,
    pub hash: u64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Solution {
    pub correct: bool,
    pub n_cliques: u16,
    pub n_cpu_cycles: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    pub inner: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunParams {
    pub graph_dist: GraphDist,
    pub algo_ids_selected: Vec<(u16, u16)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphDist {
    pub n_nodes_min: u16,
    pub n_nodes_max: u16,
    pub n_nodes_step: u16,
    pub node_degree: f64,
    pub n_iterations: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {}

impl GraphDist {
    pub fn empty() -> Self {
        Self {
            n_nodes_min: 0,
            n_nodes_max: 0,
            n_nodes_step: 0,
            node_degree: 0.0,
            n_iterations: 0,
        }
    }
}

impl Default for GraphDist {
    fn default() -> Self {
        Self {
            n_nodes_min: 100,
            n_nodes_max: 1000,
            n_nodes_step: 1,
            node_degree: 0.2,
            n_iterations: 5,
        }
    }
}
