use axum::{
    extract::{DefaultBodyLimit, Extension},
    routing::{get, post},
    Router,
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::path::{Path, PathBuf};
use tower_http::services::{ServeDir, ServeFile};

use crate::api::upload::upload_handler;
use crate::{api::route::api_handler, spust::SpustArgs};
use crate::{api::track::tracker, errors::SpustError};

#[derive(Debug, Clone)]
pub struct SpustConfig {
    pub upload_dir: PathBuf,
    pub db: Pool<Sqlite>,
}

pub async fn create_app(args: &SpustArgs) -> Result<Router, SpustError> {
    let SpustArgs {
        serve_dir,
        upload_dir,
        max_upload_size,
        database_file,
        ..
    } = args;
    let db_options = SqliteConnectOptions::new()
        .filename(database_file)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_options)
        .await?;
    let config = SpustConfig {
        upload_dir: upload_dir.clone(),
        db: pool,
    };
    let serve_dir_handler: ServeDir<tower_http::set_status::SetStatus<ServeFile>> =
        ServeDir::new(serve_dir)
            .not_found_service(ServeFile::new(Path::join(serve_dir, "index.html")));
    let upload_dir_handler = ServeDir::new(upload_dir)
        .not_found_service(ServeFile::new(Path::join(upload_dir, "index.html")));

    let router = Router::new()
        .route("/api", get(api_handler))
        .route("/api/upload", post(upload_handler))
        .layer(DefaultBodyLimit::max(*max_upload_size))
        .route("/t/{*path}", get(tracker))
        .layer(Extension(config))
        .nest_service(&format!("/{}", upload_dir.display()), upload_dir_handler)
        .fallback_service(serve_dir_handler);
    Ok(router)
}
