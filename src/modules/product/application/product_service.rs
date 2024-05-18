use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
}

pub struct ProductService {
    // Dependencies for product service
}

impl ProductService {
    pub fn new() -> Self {
        ProductService {}
    }

    pub async fn add_product(&self) -> Product {
        Product { id: 3, name: "Apple".to_string() }
    }

    pub async fn list_products(&self) -> Vec<Product> {
        vec![
            Product { id: 3, name: "Apple".to_string() }
        ]
    }
}
