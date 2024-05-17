pub mod application;

use actix_web::{web, HttpResponse}; 
use application::product_service;

pub fn module(cfg: &mut web::ServiceConfig) {
    let product_service = product_service::ProductService::new();

    cfg.service(
        web::resource("/products")
            .route(web::get().to(|| {
                let message = product_service.list_products();
                async move { HttpResponse::Ok().body(message) }
            }))
            .route(web::post().to(|| {
                let message = product_service.add_product();
                async move { HttpResponse::Ok().body(message) }
            }))
    );
}