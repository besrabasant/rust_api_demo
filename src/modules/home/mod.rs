
use axum::{
    http::StatusCode,
    routing::{get},
    Router,
};
use std::sync::Arc;

pub fn module(router: Router) -> Router {

    let home_controller = Arc::new(HomeController::new());

    router
        .route(
            "/",
            get({
                let user_controller = home_controller.clone();
                move || async move { user_controller.index().await }
            }),
        )
}

#[derive(Clone)]
pub struct HomeController {
}

impl HomeController {
    pub fn new() -> Self {
        HomeController {  }
    }

    pub async fn index(&self) -> (StatusCode, &'static str) {
        (StatusCode::OK, "Rust Web API")
    }
}
