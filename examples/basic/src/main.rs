//! Basic example for axum-static
//! - Serves files from the `public/` directory using axum_static::static_router
//! - Use `RUST_LOG=example-basic=debug,tower_http=debug` to enable runtime logs
//! - Optional Feature: `handle_error` — gate ServeDir IO error handler via Cargo feature
//!
//! Minimal, focused example demonstrating how to use the `static_router` helper.
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        tracing::error!("error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example-basic=debug,tower_http=debug".into()),
        )
        .with(fmt::layer())
        .init();

    let app = axum_static::static_router("public");
    // The `public` directory must exist and contain static assets. This example
    // will serve files from that directory. Try creating `public/index.html`.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
