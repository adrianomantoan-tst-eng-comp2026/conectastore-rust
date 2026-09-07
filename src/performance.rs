use std::time::Instant;

use crate::graph::{Graph, Vertex};
use crate::recommendation::recommend_products;

pub fn run_performance_test(total_products: u32) {
    let mut graph = Graph::new();

    let client = Vertex::Client(1);
    let seed_product = Vertex::Product(1);

    // Produto 1 representa um item já comprado ou de interesse do cliente.
    graph.add_edge(client, seed_product, 1.0);

    // Os demais produtos são relacionados ao produto de interesse.
    for id in 2..=total_products {
        graph.add_edge(
            seed_product,
            Vertex::Product(id),
            0.5,
        );
    }

    let start = Instant::now();

    let recommendations = recommend_products(&graph, client, 2);

    let elapsed = start.elapsed();

    println!(
        "Volume: {:>6} produtos | Recomendações: {:>6} | Tempo: {:?}",
        total_products,
        recommendations.len(),
        elapsed
    );
}