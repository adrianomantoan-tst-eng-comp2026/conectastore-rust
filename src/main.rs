use conectastore_rust::graph::Graph;
use conectastore_rust::product::{Product, ProductCatalog};
use conectastore_rust::recommendation::recommend_products;

fn main() {
    let mut catalog = ProductCatalog::new();

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
        199.00,
    ));

    catalog.add_product(Product::new(
        3,
        "Teclado Mecânico",
        "Informática",
        415.00,
    ));

    catalog.add_product(Product::new(
        4,
        "Monitor 27 Polegadas",
        "Informática",
        1790.00,
    ));

    catalog.add_product(Product::new(
        5,
        "Headset Gamer",
        "Áudio",
        383.00,
    ));

    let mut graph = Graph::new();

    graph.add_edge(1, 2, 0.9);
    graph.add_edge(1, 3, 0.8);
    graph.add_edge(2, 4, 0.7);
    graph.add_edge(3, 5, 0.6);

    let product_id = 1;

    println!("=== ConectaStore ===");

    if let Some(product) = catalog.get_product(product_id) {
        println!(
            "Produto selecionado: {} - {}",
            product.id, product.name
        );
    }

    println!("\nProdutos recomendados:");

    let recommendations = recommend_products(&graph, product_id, 2);

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
}