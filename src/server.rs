use axum::{
    extract::{DefaultBodyLimit, Extension},
    routing::{get, post},
    Router,
};
use std::path::{Path, PathBuf};
use tower_http::services::{ServeDir, ServeFile};

use crate::api::track::tracker;
use crate::api::upload::upload_handler;
use crate::{api::route::api_handler, spust::SpustArgs};

#[derive(Debug, Clone)]
pub struct SpustConfig {
    pub upload_dir: PathBuf,
}

pub fn create_app(args: SpustArgs) -> Router {
    let SpustArgs {
        serve_dir,
        upload_dir,
        max_upload_size,
        ..
    } = &args;
    let config = SpustConfig {
        upload_dir: upload_dir.clone(),
    };
    let serve_dir_handler: ServeDir<tower_http::set_status::SetStatus<ServeFile>> =
        ServeDir::new(serve_dir)
            .not_found_service(ServeFile::new(Path::join(serve_dir, "index.html")));
    let upload_dir_handler = ServeDir::new(upload_dir)
        .not_found_service(ServeFile::new(Path::join(upload_dir, "index.html")));

    Router::new()
        .route("/api", get(api_handler))
        .route("/api/upload", post(upload_handler))
        .layer(DefaultBodyLimit::max(*max_upload_size))
        .route("/t/{*path}", get(tracker))
        .layer(Extension(config))
        .nest_service(&format!("/{}", upload_dir.display()), upload_dir_handler)
        .fallback_service(serve_dir_handler)
}
