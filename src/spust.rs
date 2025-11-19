use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;

use crate::errors::SpustError;
use crate::server::create_app;

const DEFAULT_MAX_UPLOAD: usize = 1024 * 1024 * 10;

#[derive(Debug, Clone, Parser)]
pub struct SpustArgs {
    /// Directory to serve files from
    #[arg(long, default_value = "public")]
    pub serve_dir: PathBuf,

    /// Directory to store uploaded files
    #[arg(long, default_value = "files")]
    pub upload_dir: PathBuf,

    /// Database file
    #[arg(long, default_value = "spust.sqlite3")]
    pub database_file: PathBuf,

    /// Maximum upload size in bytes
    #[arg(long, default_value_t = DEFAULT_MAX_UPLOAD)]
    pub max_upload_size: usize,

    /// Port to listen on
    #[arg(long, default_value_t = 3000)]
    pub port: u16,
}

impl SpustArgs {
    pub async fn run_main(&self) -> Result<(), SpustError> {
        let port = self.port;
        let app = create_app(self).await?;
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("Listening on http://{}", addr);
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .map_err(|err| SpustError::new(err.to_string()))
    }
}
