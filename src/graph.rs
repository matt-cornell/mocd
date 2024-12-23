use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


pub type NodeId = i32;
pub type CommunityId = i32;
pub type Partition = HashMap<NodeId, CommunityId>;

#[derive(Debug)]
pub struct Graph {
    pub edges: Vec<(NodeId, NodeId)>,
    pub nodes: HashSet<NodeId>,
    pub adjacency_list: HashMap<NodeId, Vec<NodeId>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: HashSet::new(),
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.edges.push((from, to));
        self.nodes.insert(from);
        self.nodes.insert(to);
        
        // Update adjacency list
        self.adjacency_list
            .entry(from)
            .or_insert_with(Vec::new)
            .push(to);
        self.adjacency_list
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);
    }

    pub fn from_edgelist(path: &Path) -> Result<Self, std::io::Error> {
        let mut graph = Graph::new();
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let from: NodeId = parts[0].parse().unwrap();
                let to: NodeId = parts[1].parse().unwrap();
                graph.add_edge(from, to);
            }
        }
        Ok(graph)
    }

    pub fn neighbors(&self, node: &NodeId) -> Vec<NodeId> {
        self.adjacency_list
            .get(node)
            .cloned()
            .unwrap_or_default()
    }
}