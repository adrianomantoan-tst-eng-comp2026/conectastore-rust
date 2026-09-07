use conectastore_rust::graph::Graph;
use conectastore_rust::product::{Product, ProductCatalog};
use conectastore_rust::recommendation::recommend_products;

#[test]
fn should_register_and_find_product() {
    let mut catalog = ProductCatalog::new();

    catalog.add_product(Product::new(
        1,
        "Notebook Gamer",
        "Informática",
        4500.00,
    ));

    let product = catalog.get_product(1);

    assert!(product.is_some());
    assert_eq!(product.unwrap().name, "Notebook Gamer");
}

#[test]
fn should_return_recommendations_without_duplicates() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 0.9);
    graph.add_edge(1, 3, 0.8);
    graph.add_edge(2, 4, 0.7);
    graph.add_edge(3, 4, 0.6);

    let recommendations = recommend_products(&graph, 1, 2);

    assert_eq!(recommendations.len(), 3);
    assert!(recommendations.contains(&2));
    assert!(recommendations.contains(&3));
    assert!(recommendations.contains(&4));
}