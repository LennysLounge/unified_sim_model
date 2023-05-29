use std::{env, thread, time::Duration};

use tracing::{debug, error, info, Level};
use unified_sim_model::adapter::Adapter;

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
    let acc_adapter = Adapter::new_acc();

    loop {
        if acc_adapter.join_handle.is_finished() {
            break;
        }

        let model = match acc_adapter.model.read() {
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

        debug!("Clearing model of events");
        match acc_adapter.model.write() {
            Ok(mut model) => model.events.clear(),
            Err(e) => {
                error!("Model was poisoned: {:?}", e);
                break;
            }
        };

        thread::sleep(Duration::from_millis(1000));
    }

    if let Err(e) = acc_adapter
        .join_handle
        .join()
        .expect("Couldnt join connection thread")
    {
        info!("Connection failed because: {}", e);
    }
    info!("Connection done");
}
