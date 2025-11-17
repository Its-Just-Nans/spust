use clap_builder::Parser;
use std::process::exit;

use spust::{run_main, spust::SpustArgs};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = SpustArgs::parse();
    if let Err(e) = run_main(config).await {
        println!("{:?}", e);
        exit(1)
    };
}
