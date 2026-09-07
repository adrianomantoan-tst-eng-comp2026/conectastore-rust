use conectastore_rust::graph::{Graph, Vertex};
use conectastore_rust::performance::run_performance_test;
use conectastore_rust::product::{Product, ProductCatalog};
use conectastore_rust::recommendation::recommend_products;

fn main() {
    let mut catalog = ProductCatalog::new();

    // Cadastro dos produtos
    catalog.add_product(Product::new(
        1,
        "Notebook Gamer",
        "Informática",
        7990.00,
    ));

    catalog.add_product(Product::new(
        2,
        "Mouse Gamer",
        "Informática",
        299.00,
    ));

    catalog.add_product(Product::new(
        3,
        "Teclado Mecânico",
        "Informática",
        299.00,
    ));

    catalog.add_product(Product::new(
        4,
        "Monitor 27 Polegadas",
        "Informática",
        1699.00,
    ));

    catalog.add_product(Product::new(
        5,
        "Headset Gamer",
        "Áudio",
        250.00,
    ));

    // Construção do grafo
    let mut graph = Graph::new();

    let client = Vertex::Client(100);

    // Relações entre o cliente e produtos
    graph.add_edge(client, Vertex::Product(1), 1.0);
    graph.add_edge(client, Vertex::Product(2), 0.9);

    // Relações entre produtos
    graph.add_edge(
        Vertex::Product(1),
        Vertex::Product(3),
        0.8,
    );

    graph.add_edge(
        Vertex::Product(2),
        Vertex::Product(4),
        0.7,
    );

    graph.add_edge(
        Vertex::Product(3),
        Vertex::Product(5),
        0.6,
    );

    // Demonstração do sistema
    println!("=== ConectaStore ===");
    println!("Cliente selecionado: 100");

    println!("\nProdutos recomendados:");

    let recommendations = recommend_products(&graph, client, 2);

    for id in recommendations {
        if let Some(product) = catalog.get_product(id) {
            println!(
                "- {} | {} | {} | R$ {:.2}",
                product.id,
                product.name,
                product.category,
                product.price
            );
        }
    }

    // Testes de desempenho
    println!("\n=== Teste de Desempenho ===");

    run_performance_test(1_000);
    run_performance_test(10_000);
    run_performance_test(50_000);
}