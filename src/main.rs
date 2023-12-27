use std::env;
use tokio::net::TcpListener;
use axum::Router;

mod services;

// setup constants
const PORT: &str = "8080";
const HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let (port, host) = from_env();
    let addr = format!("{}:{}", host, port);

    let routes_all = Router::new()
        .merge(services::routes_public());

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap()
}

/// variables from environment or default to configure server
/// port, host
fn from_env() -> (String, String) {
    (
        env::var("PORT")
            .ok()
            .unwrap_or_else(|| PORT.to_string()),
        env::var("SERVER_HOST")
            .ok()
            .unwrap_or_else(|| HOST.to_string()),
    )
}