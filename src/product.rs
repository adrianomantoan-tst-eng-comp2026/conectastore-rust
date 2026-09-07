use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u32, name: &str, category: &str, price: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: category.to_string(),
            price,
        }
    }
}

pub struct ProductCatalog {
    products: HashMap<u32, Product>,
}

impl ProductCatalog {
    pub fn new() -> Self {
        Self {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(product.id, product);
    }

    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn total_products(&self) -> usize {
        self.products.len()
    }
}