mod handlers;

use axum::Router;
use axum::routing::get;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(handlers::public::index))
}