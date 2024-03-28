use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/public"]
#[prefix = "/"]
struct Assets;

struct NotFoundError;

impl IntoResponse for NotFoundError {
    fn into_response(self) -> Response {
        let error_page = Assets::get("/html/error.html").unwrap();
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(error_page.data))
            .unwrap()
    }
}

pub fn routes_public() -> Router {
    Router::new().fallback_service(get(public_handler))
}

pub async fn public_handler(uri: Uri) -> Result<impl IntoResponse, impl IntoResponse> {
    let path = match uri.path() {
        "/" => "/html/index.html",
        "/chess" => "/html/chess.html",
        "/cmu-15-418-s24-final-project" => "/html/cmu-15-418-s24-final-project.html",
        _ => uri.path(),
    };

    let mime_type = match path.rsplit('.').next() {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("png") => "image/x-png",
        Some("pdf") => "application/pdf",
        Some("jpeg") => "image/jpeg",
        _ => return Err(NotFoundError),
    };

    let asset = Assets::get(&path).ok_or(NotFoundError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type)
        .body(Body::from(asset.data))
        .unwrap())
}
