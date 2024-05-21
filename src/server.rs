use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::api::route::api_handler;
use crate::api::upload::upload_handler;
use axum::{
    extract::{DefaultBodyLimit, Extension},
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, Clone)]
pub struct SpustConfig {
    pub serve_dir: PathBuf,
    pub upload_dir: PathBuf,
    pub max_upload_size: usize,
    pub port: u16,
}

pub fn create_app(config: SpustConfig) -> Router {
    let arced = Arc::new(config.clone());
    let SpustConfig {
        serve_dir,
        upload_dir,
        max_upload_size,
        ..
    } = config;
    let serve_dir_handler = ServeDir::new(&arced.serve_dir)
        .not_found_service(ServeFile::new(Path::join(&serve_dir, "index.html")));
    let upload_dir_handler = ServeDir::new(&arced.upload_dir)
        .not_found_service(ServeFile::new(Path::join(&upload_dir, "index.html")));

    Router::new()
        .route("/api", get(api_handler))
        .route("/api/upload", post(upload_handler))
        .layer(Extension(arced))
        .layer(DefaultBodyLimit::max(max_upload_size))
        .route("/api/*path", get(api_handler))
        .nest_service(
            &format!("/{}", upload_dir.to_str().unwrap()),
            upload_dir_handler,
        )
        .fallback_service(serve_dir_handler)
}
