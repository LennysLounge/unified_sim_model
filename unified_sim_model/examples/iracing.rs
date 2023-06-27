use std::env;

use tracing::{info, Level};
use unified_sim_model::{Adapter, AdapterCommand};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set global subscriber");

    info!("Connecting to game");
    let mut adapter = Adapter::new_iracing();

    // Wait for an update and loop.
    let mut limit = 0;
    while adapter.wait_for_update().is_ok() {
        let model = adapter.model.read().unwrap();

        if let Some(session) = model.current_session() {
            info!(
                "Session time: {}, Session time remaining: {}",
                session.session_time, session.time_remaining
            );
        };

        for event in model.events.iter() {
            info!("Event: {:?}", event);
        }
        std::mem::drop(model);
        _ = adapter.clear_events();

        limit += 1;
        if limit > 100 {
            adapter.send(AdapterCommand::Close);
            break;
        }
    }

    if let Some(Err(e)) = adapter.join() {
        info!("Connection failed because: {}", e);
    }

    info!("Connection done");
}
