use axum::{
    routing::{get, get_service, post},
    Router,
};
use tower_http::services::ServeDir;

use super::FRONT_PUBLIC;

/// Front end to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    Router::new()
        .fallback_service(get_service(ServeDir::new(FRONT_PUBLIC)))
}