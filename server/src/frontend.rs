use axum::http::{header, HeaderMap, StatusCode, Uri};
use rust_embed::RustEmbed;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../app/dist/"]
struct EmbeddedFrontend;

type FrontendResponse = Result<(HeaderMap, Vec<u8>), StatusCode>;

fn serve_file(path: &str) -> FrontendResponse {
    let file = EmbeddedFrontend::get(path).ok_or(StatusCode::NOT_FOUND)?;

    let mut headers = HeaderMap::new();
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    headers.append(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());

    Ok((headers, file.data.into()))
}

pub async fn frontend_handler(uri: Uri) -> FrontendResponse {
    let path = uri.path().trim_start_matches('/');
    serve_file(path).or_else(|_| serve_file(INDEX_HTML))
}
