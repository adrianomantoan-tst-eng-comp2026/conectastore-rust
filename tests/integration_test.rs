use conectastore_rust::graph::{Graph, Vertex};
use conectastore_rust::product::{Product, ProductCatalog};
use conectastore_rust::recommendation::recommend_products;

#[test]
fn should_register_and_find_product() {
    let mut catalog = ProductCatalog::new();

    catalog.add_product(Product::new(
        1,
        "Notebook Gamer",
        "Informática",
        7990.00,
    ));

    let product = catalog.get_product(1);

    assert!(product.is_some());
    assert_eq!(product.unwrap().name, "Notebook Gamer");
}

#[test]
fn should_return_recommendations_without_duplicates() {
    let mut graph = Graph::new();

    let client = Vertex::Client(100);

    graph.add_edge(client, Vertex::Product(1), 1.0);
    graph.add_edge(client, Vertex::Product(2), 0.9);

    graph.add_edge(Vertex::Product(1), Vertex::Product(3), 0.8);
    graph.add_edge(Vertex::Product(2), Vertex::Product(3), 0.7);
    graph.add_edge(Vertex::Product(2), Vertex::Product(4), 0.6);

    let recommendations = recommend_products(&graph, client, 2);

    assert_eq!(recommendations.len(), 2);
    assert!(recommendations.contains(&3));
    assert!(recommendations.contains(&4));

    assert!(!recommendations.contains(&1));
    assert!(!recommendations.contains(&2));
}

#[test]
fn should_recommend_from_product() {
    let mut graph = Graph::new();

    graph.add_edge(
        Vertex::Product(1),
        Vertex::Product(2),
        0.9,
    );

    graph.add_edge(
        Vertex::Product(2),
        Vertex::Product(3),
        0.8,
    );

    let recommendations =
        recommend_products(&graph, Vertex::Product(1), 2);

    assert_eq!(recommendations.len(), 2);
    assert!(recommendations.contains(&2));
    assert!(recommendations.contains(&3));
}