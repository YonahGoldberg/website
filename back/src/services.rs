use axum::{
    Router,
    response::{ IntoResponse, Response },
    http::{header, StatusCode, Uri}, body::Body, routing::get,
};

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/public"]
#[prefix = "/"]
struct Assets;

struct NotFoundError;

impl IntoResponse for NotFoundError {
    fn into_response(self) -> Response {
        let error_page = Assets::get("/error.html").unwrap();
        Response::builder()
            .status(StatusCode::NOT_FOUND)        
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(error_page.data))
            .unwrap()
    }
}

pub fn routes_public() -> Router {
    Router::new()
        .fallback_service(get(public_handler))
}

pub async fn public_handler(uri: Uri) -> Result<impl IntoResponse, impl IntoResponse> {
    let path = if uri.path() == "/" { "/index.html" } else { uri.path() };

    let mime_type = match path.rsplit('.').next() {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => return Err(NotFoundError)
    };
    
    let asset = Assets::get(&path).ok_or(NotFoundError)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime_type)
        .body(Body::from(asset.data))
        .unwrap())
}