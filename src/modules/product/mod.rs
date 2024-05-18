pub mod application;

use self::application::product_service::{Product, ProductService};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

pub fn module(router: Router) -> Router {
    let product_service = Arc::new(ProductService::new());

    let product_controller = Arc::new(ProductController::new(product_service));

    router
        .route(
            "/products",
            get({
                let product_controller = product_controller.clone();
                move || async move { product_controller.index().await }
            }),
        )
        .route(
            "/products",
            post({
                let product_controller = product_controller.clone();
                move || async move { product_controller.store().await }
            }),
        )
}

#[derive(Clone)]
pub struct ProductController {
    pub product_service: Arc<ProductService>,
}

impl ProductController {
    pub fn new(product_service: Arc<ProductService>) -> Self {
        ProductController { product_service }
    }

    pub async fn index(&self) -> (StatusCode, Json<Vec<Product>>) {
        let products = self.product_service.list_products().await;
        (StatusCode::OK, Json(products))
    }

    pub async fn store(&self) -> (StatusCode, Json<Product>) {
        let product = self.product_service.add_product().await;
        (StatusCode::OK, Json(product))
    }
}
