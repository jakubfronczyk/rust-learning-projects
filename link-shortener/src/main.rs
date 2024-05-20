use std::error::Error;

use axum::{
    routing::{get, patch, post},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use dotenvy::dotenv;
use routes::health;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::{create_link, get_link_statistics, redirect, update_link};

mod routes;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file if it exists
    dotenv().ok();

    // Setup tracing
    tracing_subscriber::registry()
        // Set the filter level based on the environment variable or default to "link_shortener=debug"
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "link_shortener=debug".into()),
        )
        // Add the fmt layer for formatting logs
        .with(tracing_subscriber::fmt::layer())
        // Initialize the tracing subscriber
        .init();

    // Read the db url
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL is a required environment variables");

    // Create a connection pool
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    // Create a new instance of PrometheusMetricLayer and get a handle to metrics
    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    // Create the Axum application
    let app = Router::new()
        .route("/create", post(create_link))
        .route("/:id/statistics", get(get_link_statistics))
        .route("/:id", patch(update_link).get(redirect))
        // Define a route for "/metrics" endpoint to serve Prometheus metrics
        .route("/metrics", get(|| async move { metrics_handle.render() }))
        // Define a route for "/health" endpoint using the health function imported from routes module
        .route("/health", get(health))
        // Add TraceLayer for HTTP tracing
        .layer(TraceLayer::new_for_http())
        // Add PrometheusMetricLayer for metrics
        .layer(prometheus_layer)
        //
        .with_state(db);

    // Bind a TCP listener to address "0.0.0.0:3000"
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not initialize TcpListener");

    // Print debug message indicating the address the server is listening on
    tracing::debug!(
        "listening on {}",
        listener
            .local_addr()
            .expect("Could not convert listener address to local address")
    );

    // Start the Axum server to serve the application
    axum::serve(listener, app)
        .await
        .expect("Could not successfully create server");

    // Return Ok(()) to indicate successful execution of the main function
    Ok(())
}
