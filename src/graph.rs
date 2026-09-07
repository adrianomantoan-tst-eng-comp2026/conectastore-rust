use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Edge {
    pub destination: u32,
    pub weight: f64,
}

pub struct Graph {
    adjacency_list: HashMap<u32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex_id: u32) {
        self.adjacency_list.entry(vertex_id).or_default();
    }

    pub fn add_edge(&mut self, source: u32, destination: u32, weight: f64) {
        self.add_vertex(source);
        self.add_vertex(destination);

        if let Some(edges) = self.adjacency_list.get_mut(&source) {
            edges.push(Edge {
                destination,
                weight,
            });
        }
    }

    pub fn neighbors(&self, vertex_id: u32) -> Option<&Vec<Edge>> {
        self.adjacency_list.get(&vertex_id)
    }

    pub fn total_vertices(&self) -> usize {
        self.adjacency_list.len()
    }
}