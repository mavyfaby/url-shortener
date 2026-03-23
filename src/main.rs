mod config;
mod infrastructure;

use config::AppConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig::load();
    let router = infrastructure::http::create_router();

    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect(&format!("Cannot bind to port {}.", config.port));

    println!("URL Shortener is listening on port {}!", config.port);

    axum::serve(listener, router).await
        .expect("An error occurred while running the server.")
}
