use spust::{run_main, server::SpustConfig};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = SpustConfig {
        serve_dir: "dist".into(),
        upload_dir: "files".into(),
        max_upload_size: 1024 * 1024 * 10,
        port: 3000,
    };
    match run_main(config).await {
        Err(e) => {
            println!("{:?}", e);
        }
        _ => {}
    };
}
