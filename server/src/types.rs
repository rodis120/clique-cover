#[allow(unused_imports)]

use std::fmt;
use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use tokio::sync::RwLock;
use shared::{Solution, Graph, GraphDist, Summary};

pub struct SharedState {
    pub session_id: AtomicU8,
    pub algorithms: RwLock<Vec<(u16, String)>>,
    pub algos_in_use: RwLock<Vec<u16>>,
    pub graph_dist: RwLock<GraphDist>,
    pub database: RwLock<Database>,
}

#[derive(Debug)]
pub struct Database {
    size: u64,
    h_size: u16,
    v_size: u16,
    n_algos: u16,
    graphs: Arc<[RwLock<Option<Graph>>]>,
    solutions: Arc<[RwLock<Option<Solution>>]>,
    graph_order: RwLock<Vec<u32>>,
    solution_order: RwLock<Vec<u64>>,
    summary: RwLock<Option<Summary>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            session_id: AtomicU8::new(0),
            algos_in_use: RwLock::new(Vec::new()),
            algorithms: RwLock::new(Vec::new()),
            graph_dist: RwLock::new(GraphDist::default()),
            database: RwLock::new(Database::new(0, GraphDist::default())),
        }
    }
}

impl Database {
    pub fn new(n_algos: u16, graph_dist: GraphDist) -> Self {
        let v_size = 
            (graph_dist.n_nodes_max - graph_dist.n_nodes_min)
            / graph_dist.n_nodes_step
            + 1;
        let h_size = graph_dist.n_iterations;
        let size = v_size as u64 * h_size as u64 * n_algos as u64;

        let graphs_vec: Vec<RwLock<Option<Graph>>> =
            (0..h_size * v_size)
                .map(|_| RwLock::new(None))
                .collect();

        let solutions_vec: Vec<RwLock<Option<Solution>>> =
            (0..size)
                .map(|_| RwLock::new(None))
                .collect();

        let graph_order= RwLock::new(Vec::<u32>::with_capacity(size as usize));
        let solution_order = RwLock::new(Vec::<u64>::with_capacity(size as usize));

        Self {
            size: size,
            h_size,
            v_size,
            n_algos,
            graphs: Arc::from(graphs_vec.into_boxed_slice()),
            solutions: Arc::from(solutions_vec.into_boxed_slice()),
            graph_order,
            solution_order,
            summary: RwLock::new(None),
        }
    }

    pub async fn insert_graph(&self, graph: Graph, graph_id: u32) {
        if let Some(lock) = self.graphs.get(graph_id as usize) {
            let mut guard = lock.write().await;
            *guard = Some(graph);
        } else {
            eprintln!("insert_graph(): id out of bounds");
        }
    }

    pub async fn get_graph(&self, graph_id: u32) -> Option<Graph> {
        if let Some(lock) = self.graphs.get(graph_id as usize) {
            let guard = lock.read().await;
            guard.clone()
        } else {
            eprintln!("get_graph(): id out of bounds");
            None
        }
    }

    pub async fn get_graph_order(&self) -> Vec<u32> {
        let mut guard = self.graph_order.read().await;
        guard.clone()
    }

    fn get_solution_index(&self, algo_id: u16, graph_id: u32) -> usize {
        (algo_id as usize) 
            * (self.h_size as usize) 
            * (self.v_size as usize) 
            + (graph_id as usize)
    }

    pub async fn insert_solution(&self, solution: Solution, algo_id: u16, graph_id: u32) {
        let index = self.get_solution_index(algo_id, graph_id);
        if let Some(lock) = self.solutions.get(index) {
            let mut guard = lock.write().await;
            *guard = Some(solution);
        } else {
            eprintln!("insert_solution(): id out of bounds");
        }
    }

    pub async fn get_solution(&self, algo_id: u16, graph_id: u32) -> Option<Solution> {
        let index = self.get_solution_index(algo_id, graph_id);
        if let Some(lock) = self.solutions.get(index) {
            let guard = lock.read().await;
            guard.clone()
        } else {
            eprintln!("get_solution(): id out of bounds");
            None
        }
    }

    pub async fn get_solution_order(&self) -> Vec<u64> {
        let mut guard = self.solution_order.read().await;
        guard.clone()
    }

    pub fn get_algo_and_graph_id(&self, index: u64) -> (u16, u32) {
       let algo_id = index / (self.h_size as u64 * self.v_size as u64);
       let graph_id = index % (self.h_size as u64 * self.v_size as u64);
       (algo_id as u16, graph_id as u32)
    }

    pub fn get_primitive_fields(&self) -> (u64, u16, u16, u16) {
        (self.size, self.h_size, self.v_size, self.n_algos)
    }

    pub async fn print(&self) {
        println!("Database:");
        println!("  size: {}", self.size);
        println!("  h_size: {}", self.h_size);
        println!("  v_size: {}", self.v_size);
        println!("  n_algos: {}", self.n_algos);
        println!("  graphs:");
        for g in self.graphs.iter() {
            let guard = g.read().await;
            match &*guard {
                Some(g) => println!("    graph({})", g),
                None => println!("    nothing"),
            };
        }
    }
}
