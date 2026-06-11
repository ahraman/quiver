use std::sync::Arc;

use axum::{Router, debug_handler, response::IntoResponse, routing::get};
use tower_http::services::ServeDir;

use crate::AppState;

impl super::App {
    pub fn build_router(self: &Arc<Self>) -> Router {
        build_global_router().with_state(AppState(self.clone()))
    }
}

fn build_global_router() -> Router<AppState> {
    Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("assets/public"))
}

#[debug_handler(state = AppState)]
async fn root(_: AppState) -> impl IntoResponse {
    "Hello, world!"
}
