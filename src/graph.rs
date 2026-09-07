use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vertex {
    Client(u32),
    Product(u32),
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub destination: Vertex,
    pub weight: f64,
}

pub struct Graph {
    adjacency_list: HashMap<Vertex, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.adjacency_list.entry(vertex).or_default();
    }

    pub fn add_edge(&mut self, source: Vertex, destination: Vertex, weight: f64) {
        self.add_vertex(source);
        self.add_vertex(destination);

        if let Some(edges) = self.adjacency_list.get_mut(&source) {
            edges.push(Edge {
                destination,
                weight,
            });
        }
    }

    pub fn neighbors(&self, vertex: Vertex) -> Option<&Vec<Edge>> {
        self.adjacency_list.get(&vertex)
    }

    pub fn total_vertices(&self) -> usize {
        self.adjacency_list.len()
    }
}