use std::{env, thread, time::Duration};

use tracing::{error, info, Level};
use unified_sim_model::adapter::{Adapter, AdapterAction};

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
    let mut adapter = Adapter::new_acc();

    loop {
        if adapter.is_finished() {
            break;
        }

        let model = match adapter.model.read() {
            Ok(lock) => lock,
            Err(e) => {
                error!("Model was poisoned: {:?}", e);
                break;
            }
        };

        if let Some(session) = model.current_session() {
            info!(
                "Session time: {}, Session time remaining: {}",
                session.session_time, session.time_remaining
            );
        };

        for event in model.events.iter() {
            info!("Event: {:?}", event);
        }
        drop(model);

        if let Err(e) = adapter.sender.send(AdapterAction::ClearEvents) {
            error!("Cannot send to adapter: {}", e);
        }

        thread::sleep(Duration::from_millis(1000));
    }

    if let Some(Err(e)) = adapter.take_result() {
        info!("Connection failed because: {}", e);
    }
    info!("Connection done");
}
