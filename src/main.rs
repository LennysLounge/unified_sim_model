use std::env;

use tracing::{info, Level};
use unified_sim_model::acc::AccAdapter;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set global subscriber");

    info!("Connecting to game");
    let acc_adapter = AccAdapter::new().expect("Cannot connect to game");

    if let Err(e) = acc_adapter
        .join_handle
        .join()
        .expect("Couldnt join connection thread")
    {
        info!("Connection failed because: {}", e);
    }
    info!("Connection done");
}
