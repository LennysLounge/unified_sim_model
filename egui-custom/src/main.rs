use std::{cell::RefCell, env, rc::Rc};

use egui_custom::{run_event_loop, window::WindowOptions};
use test_app::TestApp;
use tracing::Level;
use winit::dpi::{PhysicalSize, Size};

mod test_app;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "");

    env_logger::init();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set global subscriber");

    run_event_loop(
        WindowOptions {
            title: "Test window".to_string(),
            size: Some(Size::Physical(PhysicalSize {
                width: 340,
                height: 200,
            })),
            ..Default::default()
        },
        Box::new(|| Rc::new(RefCell::new(TestApp::default()))),
    );
}
