use std::path::Path;
use std::sync::Arc;

use axum::extract::Request;
use axum::http::header::{self, CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::HeaderMap;
use axum::Extension;
use axum::{body::Body, http::StatusCode, response::IntoResponse};
use tokio_util::io::ReaderStream;

use crate::server::SpustConfig;

fn get_mime(filename: impl AsRef<Path>) -> mime::Mime {
    return mime_guess::from_path(filename).first_or_octet_stream();
}

pub async fn tracker(
    Extension(conf): Extension<Arc<SpustConfig>>,
    req: Request,
) -> impl IntoResponse {
    let url = req.uri().path();
    // remove the first character '/t'
    let filename = &url[3..];
    println!("URL: {}", filename);
    let content_type = get_mime(&filename);
    let file = match tokio::fs::File::open(Path::join(&conf.upload_dir, &filename)).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    match header::HeaderValue::from_str(format!("{}", content_type).as_str()) {
        Ok(value) => {
            headers.insert(CONTENT_TYPE, value);
        }
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err))),
    }

    match header::HeaderValue::from_str(format!("attachment; filename=\"{}\"", filename).as_str()) {
        Ok(value) => {
            headers.insert(CONTENT_DISPOSITION, value);
        }
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err))),
    }

    Ok((headers, body))
}
