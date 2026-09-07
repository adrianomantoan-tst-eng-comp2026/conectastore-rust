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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_product() {
        let product = Product::new(
            1,
            "Notebook Gamer",
            "Informática",
            7990.00,
        );

        assert_eq!(product.id, 1);
        assert_eq!(product.name, "Notebook Gamer");
        assert_eq!(product.category, "Informática");
        assert_eq!(product.price, 7990.00);
    }

    #[test]
    fn should_add_product_to_catalog() {
        let mut catalog = ProductCatalog::new();

        catalog.add_product(Product::new(
            1,
            "Mouse Gamer",
            "Informática",
            299.00,
        ));

        assert_eq!(catalog.total_products(), 1);
        assert!(catalog.get_product(1).is_some());
    }
}
