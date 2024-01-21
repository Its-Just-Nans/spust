use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use spust::{api_handler, upload_handler};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    const DIST: &str = "dist";
    let serve_dir =
        ServeDir::new(DIST).not_found_service(ServeFile::new(format!("{DIST}/index.html")));
    const FILES: &str = "files";
    let files_dir =
        ServeDir::new(FILES).not_found_service(ServeFile::new(format!("{FILES}/index.html")));

    let app = Router::new()
        .route("/api", get(api_handler))
        .route("/api/upload", post(upload_handler))
        .layer(DefaultBodyLimit::max(5 * 1024))
        .route("/api/*path", get(api_handler))
        .nest_service(&format!("/{FILES}"), files_dir)
        .fallback_service(serve_dir);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await;
    if let Err(e) = listener {
        println!("Error: {} {}", e, addr);
        return;
    }
    let listener = listener.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
