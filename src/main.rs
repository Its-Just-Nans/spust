use clap_builder::Parser;
use std::process::exit;

use spust::SpustArgs;

#[tokio::main]
async fn main() {
    env_logger::init();

    let spust = SpustArgs::parse(); // parse can exit
    if let Err(e) = spust.run_main().await {
        println!("{:?}", e);
        exit(1)
    };
}
