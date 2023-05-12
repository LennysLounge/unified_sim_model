use tracing::{debug, error, info, trace, warn, Level};

use crate::acc_udp::AccAdapter;

mod acc_udp;
mod messages;
mod model;

fn main() {
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
