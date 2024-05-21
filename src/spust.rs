use std::net::SocketAddr;

use crate::{create_app, server::SpustConfig};

pub async fn run_main(config: SpustConfig) -> Result<(), std::io::Error> {
    let port = config.port;
    let app = create_app(config);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await;
    let listener = match listener {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    println!("Listening on http://{}", addr);
    return axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await;
}
