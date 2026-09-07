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
                let next_depth = depth + 1;

                if visited.insert(next) {
                    queue.push_back((next, next_depth));

                    if let Vertex::Product(product_id) = next {
                        let should_recommend = match start {
                            Vertex::Client(_) => next_depth > 1,
                            Vertex::Product(_) => true,
                        };

                        if should_recommend {
                            recommendations.push(product_id);
                        }
                    }
                }
            }
        }
    }

    recommendations
}