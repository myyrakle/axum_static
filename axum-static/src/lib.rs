//! # `axum_static`
//!
//! Static file serving for Axum.
//!
//! This crate provides utilities for serving static files with proper content-type inference.
//!
//! ## Features
//!
//! - `handle_error`: Enables error handling for IO errors when serving files.
//! - `mime_guess`: Uses the `mime_guess` crate for exhaustive MIME inference.
//! - `status_code`: Enhances error responses with human-readable status messages.
//!
//! ## Example
//!
//! ```rust
//! use axum_static::static_router;
//!
//! let app = static_router("static/");
//! ```

#[cfg(feature = "handle_error")]
use axum::http::StatusCode;
#[cfg(feature = "handle_error")]
use axum::response::IntoResponse;
use axum::{
    Router,
    body::Body,
    http::Request,
    middleware::{Next, from_fn},
    response::Response,
    routing::get_service,
};
#[cfg(all(feature = "handle_error", feature = "status_code"))]
use status_code::statuses;
#[cfg(feature = "handle_error")]
use std::io;
use std::path::Path;
use tower_http::services::ServeDir;

#[cfg(feature = "tracing")]
use tracing::{error, warn};

#[cfg(not(feature = "mime_guess"))]
fn infer_content_type_from_extension(extension: &str) -> &'static str {
    match extension {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "ttf" => "font/ttf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "eot" => "application/vnd.ms-fontobject",
        "otf" => "font/otf",
        "txt" => "text/plain",
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "xml" => "application/xml",
        "zip" => "application/zip",
        "rar" => "application/x-rar-compressed",
        "7z" => "application/x-7z-compressed",
        "gz" => "application/gzip",
        "tar" => "application/x-tar",
        "swf" => "application/x-shockwave-flash",
        "flv" => "video/x-flv",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mp4" | "f4v" | "f4p" | "f4a" | "f4b" => "video/mp4",
        "mp3" => "audio/mpeg",
        "wav" => "audio/x-wav",
        "ogg" => "audio/ogg",
        "webm" => "video/webm",
        "mpg" | "mpeg" | "mpe" | "mp2" => "video/mpeg",
        "m4v" => "video/x-m4v",
        "3gp" => "video/3gpp",
        "3g2" => "video/3gpp2",
        "mkv" | "amv" => "video/x-matroska",
        "m3u" => "audio/x-mpegurl",
        "m3u8" => "application/vnd.apple.mpegurl",
        "ts" => "video/mp2t",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "tif" | "tiff" => "image/tiff",
        "psd" => "image/vnd.adobe.photoshop",
        "ai" | "eps" | "ps" => "application/postscript",
        "dwg" => "image/vnd.dwg",
        "dxf" => "image/vnd.dxf",
        "rtf" => "application/rtf",
        "odt" => "application/vnd.oasis.opendocument.text",
        "ods" => "application/vnd.oasis.opendocument.spreadsheet",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}

/// Middleware that sets the `Content-Type` header based on the file extension.
///
/// This middleware inspects the request URI's path, extracts the file extension,
/// and maps it to the appropriate MIME type. If no extension is found or it's unknown,
/// it defaults to "application/octet-stream".
///
/// Note: This does not override an existing `Content-Type` header.
pub async fn content_type_middleware(request: Request<Body>, next: Next) -> Response {
    let uri = request.uri().to_owned();
    let path = uri.path();

    // Extract the extension before awaiting to avoid holding a borrow across await points.
    let extension = path.rsplit('.').next().map(str::to_ascii_lowercase);

    let mut response = next.run(request).await;

    #[cfg(feature = "mime_guess")]
    let content_type = {
        let guessed = mime_guess::from_path(path).first_raw();
        match (guessed, extension.as_deref()) {
            (Some(mime), _) => mime,
            (None, Some(ext)) => {
                #[cfg(feature = "tracing")]
                warn!(%path, %ext, "Unknown MIME type; defaulting to application/octet-stream");
                "application/octet-stream"
            }
            (None, None) => "unknown",
        }
    };

    #[cfg(not(feature = "mime_guess"))]
    let content_type = match extension.as_deref() {
        Some(ext) => {
            let mime = infer_content_type_from_extension(ext);
            #[cfg(feature = "tracing")]
            if mime == "application/octet-stream" {
                warn!(%path, %ext, "Unknown MIME type; defaulting to application/octet-stream");
            }
            mime
        }
        None => "unknown",
    };

    if let Ok(content_type) = content_type.parse() {
        response.headers_mut().insert("Content-Type", content_type);
    }

    response
}

/// Creates a router that serves static files from the given directory.
///
/// The router uses `tower_http::services::ServeDir` to serve files, with index.html
/// appended for directories. It applies the `content_type_middleware` to set appropriate
/// content types.
///
/// # Arguments
///
/// * `path` - The path to the directory containing static files.
///
/// # Features
///
/// When the `handle_error` feature is enabled, IO errors are handled by returning
/// a 500 Internal Server Error response.
pub fn static_router<P: AsRef<Path>>(path: P) -> Router {
    /// Error handler for IO errors when serving static files.
    ///
    /// This function returns a 500 Internal Server Error response with the error message.
    ///
    /// # Arguments
    ///
    /// * `err` - The IO error that occurred.
    ///
    /// # Features
    ///
    /// This function is only available when the `handle_error` feature is enabled.
    #[cfg(feature = "handle_error")]
    async fn handle_error(err: io::Error) -> impl IntoResponse {
        #[cfg(feature = "status_code")]
        let (status, body) = {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let code = status.as_u16();
            let label = statuses::code(code);
            let body = format!("static router IO error ({} {}): {:?}", code, label, err);
            (status, body)
        };

        #[cfg(not(feature = "status_code"))]
        let (status, body) = {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let body = format!("static router IO error: {:?}", err);
            (status, body)
        };

        #[cfg(feature = "tracing")]
        {
            error!(%body, error = %err, "static router IO error");
        }

        (status, body).into_response()
    }

    let serve_dir = ServeDir::new(path).append_index_html_on_directories(true);
    #[cfg(feature = "handle_error")]
    let serve_dir = get_service(serve_dir).handle_error(handle_error);
    #[cfg(not(feature = "handle_error"))]
    let serve_dir = get_service(serve_dir);

    Router::new()
        .fallback_service(serve_dir)
        .layer(from_fn(content_type_middleware))
}
