use std::{env, net::SocketAddr};

mod services;

// setup constants
const FRONT_PUBLIC: &str = "./front_end/dist";
const SERVER_PORT: &str = "8080";
const SERVER_HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let (port, host) = from_env();

    let addr = format!("{}:{}", port, host)
        .parse::<SocketAddr>()
        .expect("Can not parse address and port");

    let app = Router::new()
        .merge(services::front_public_route());

    axum::Server::bind(&addr)
        .serve(app.make_into_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}

/// variables from environment or default to configure server
/// port, host, secret
fn from_env() -> (String, String) {
    (
        env::var("SERVER_PORT")
            .ok()
            .unwrap_or_else(|| SERVER_PORT.to_string()),
        env::var("SERVER_HOST")
            .ok()
            .unwrap_or_else(|| SERVER_HOST.to_string()),
    )
}