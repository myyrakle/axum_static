use axum::{
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::get_service,
    Router,
};
use std::path::Path;
use tower_http::services::ServeDir;

pub async fn content_type_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let uri = req.uri().to_owned();
    let path = uri.path();

    let splited = path.split(".").collect::<Vec<_>>();
    if let Some(extension) = splited.last() {
        let mut response = next.run(req).await;
        let extension = extension.to_owned().to_lowercase();

        let content_type = match extension.as_str() {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "json" => "application/json",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
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
            "mp4" => "video/mp4",
            "mp3" => "audio/mpeg",
            "wav" => "audio/x-wav",
            "ogg" => "audio/ogg",
            "webm" => "video/webm",
            "mpg" => "video/mpeg",
            "mpeg" => "video/mpeg",
            "mpe" => "video/mpeg",
            "mp2" => "video/mpeg",
            "m4v" => "video/x-m4v",
            "3gp" => "video/3gpp",
            "3g2" => "video/3gpp2",
            "mkv" => "video/x-matroska",
            "amv" => "video/x-matroska",
            "m3u" => "audio/x-mpegurl",
            "m3u8" => "application/vnd.apple.mpegurl",
            "ts" => "video/mp2t",
            "f4v" => "video/mp4",
            "f4p" => "video/mp4",
            "f4a" => "video/mp4",
            "f4b" => "video/mp4",
            "webp" => "image/webp",
            "bmp" => "image/bmp",
            "tif" => "image/tiff",
            "tiff" => "image/tiff",
            "psd" => "image/vnd.adobe.photoshop",
            "ai" => "application/postscript",
            "eps" => "application/postscript",
            "ps" => "application/postscript",
            "dwg" => "image/vnd.dwg",
            "dxf" => "image/vnd.dxf",
            "rtf" => "application/rtf",
            "odt" => "application/vnd.oasis.opendocument.text",
            "ods" => "application/vnd.oasis.opendocument.spreadsheet",
            "wasm" => "application/wasm",
            _ => "application/octet-stream",
        };

        if let Ok(content_type) = content_type.parse() {
            response.headers_mut().insert("Content-Type", content_type);
        }

        response
    } else {
        let mut response = next.run(req).await;

        if let Ok(content_type) = "application/octet-stream".parse() {
            response.headers_mut().insert("Content-Type", content_type);
        }

        response
    }
}

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

    let router = Router::new()
        .fallback_service(serve_dir)
        .layer(from_fn(content_type_middleware));

    router
}
