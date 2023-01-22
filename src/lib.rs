use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::path::Path;
use tower_http::services::ServeDir;

pub fn static_router<P: AsRef<Path>>(path: P) -> Router {
    async fn handle_error(err: std::io::Error) -> impl IntoResponse {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("static router IO error: {:?}", err),
        )
            .into_response()
    }

    let serve_dir = ServeDir::new(path);
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    let router = Router::new().fallback_service(serve_dir);

    router
}
