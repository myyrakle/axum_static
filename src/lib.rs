use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::path::Path;
use tower_http::services::ServeDir;

pub fn static_router<P: AsRef<Path>>(path: P) -> Router {
    async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
        (StatusCode::INTERNAL_SERVER_ERROR, "something wrong").into_response()
    }

    let serve_dir = ServeDir::new(path);
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    let router = Router::new().fallback_service(serve_dir);

    router
}
