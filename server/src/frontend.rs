use axum::{
    http::{header, HeaderMap, StatusCode, Uri},
    response::Html,
};
use rust_embed::RustEmbed;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../app/dist/"]
struct EmbeddedFrontend;

type FrontendResponse = Result<(HeaderMap, Html<String>), StatusCode>;

fn serve_file(path: &str) -> FrontendResponse {
    let file = EmbeddedFrontend::get(path).ok_or(StatusCode::NOT_FOUND)?;
    let content =
        String::from_utf8(file.data.into()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut headers = HeaderMap::new();
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    headers.append(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());

    Ok((headers, Html(content)))
}

pub async fn frontend_handler(uri: Uri) -> FrontendResponse {
    let path = uri.path().trim_start_matches('/');
    serve_file(path).or_else(|_| serve_file(INDEX_HTML))
}
