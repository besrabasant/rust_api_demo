pub mod application;

use self::application::user_service::{User, UserService};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

pub fn module(router: Router) -> Router {
    let user_service = Arc::new(UserService::new());

    let user_controller = Arc::new(UserController::new(user_service));

    router
        .route(
            "/users",
            get({
                let user_controller = user_controller.clone();
                move || async move { user_controller.index().await }
            }),
        )
        .route(
            "/users",
            post({
                let user_controller = user_controller.clone();
                move || async move { user_controller.store().await }
            }),
        )
}

#[derive(Clone)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> Self {
        UserController { user_service }
    }

    pub async fn index(&self) -> (StatusCode, Json<Vec<User>>) {
        let users = self.user_service.list_users().await;
        (StatusCode::OK, Json(users))
    }

    pub async fn store(&self) -> (StatusCode, Json<User>) {
        let user = self.user_service.create_user().await;
        (StatusCode::CREATED, Json(user))
    }
}
