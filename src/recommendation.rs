use std::collections::{HashSet, VecDeque};

use crate::graph::{Graph, Vertex};

pub fn recommend_products(
    graph: &Graph,
    start: Vertex,
    max_depth: usize,
) -> Vec<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut recommendations = Vec::new();

    visited.insert(start);
    queue.push_back((start, 0usize));

    while let Some((current, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        if let Some(neighbors) = graph.neighbors(current) {
            for edge in neighbors {
                let next = edge.destination;

                if visited.insert(next) {
                    queue.push_back((next, depth + 1));

                    if let Vertex::Product(product_id) = next {
                        recommendations.push(product_id);
                    }
                }
            }
        }
    }

    recommendations
}