mod app_window;
mod test_app;
mod tree;

use app_window::{AppCreator, AppWindowState};
use std::{cell::RefCell, env};
use test_app::TestApp;
use tracing::{info, Level};
use tree::Tree;
use winit::{event::WindowEvent, event_loop::EventLoop};

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

    run_event_loop(Box::new(|| Box::new(TestApp::default())));
}

fn run_event_loop(creator: AppCreator) {
    let mut windows = Tree::<RefCell<AppWindowState>>::new();

    EventLoop::new().run(move |event, event_loop, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                for node in windows.values() {
                    node.value.borrow_mut().update_redraw_timer();
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Close requested");
                control_flow.set_exit();
                #[allow(clippy::needless_return)]
                return;
            }

            // Pass window events to the apps.
            Event::WindowEvent {
                ref window_id,
                ref event,
            } => {
                for node in windows.values() {
                    node.value.borrow_mut().on_window_event(event, window_id);
                }
            }

            // Create the apps here.
            Event::Resumed => {
                windows.new_node(RefCell::new(AppWindowState::new_creator(
                    event_loop, &creator,
                )));
            }

            Event::RedrawRequested(ref window_id) => {
                for node in windows.values() {
                    node.value.borrow_mut().run_and_paint(event_loop, window_id);
                }
            }

            // At the end of the event cycle set the control flow.
            Event::RedrawEventsCleared => {
                let earliest_redraw = windows
                    .values()
                    .filter_map(|node| node.value.borrow().get_redraw_timer())
                    .min();

                if let Some(time) = earliest_redraw {
                    control_flow.set_wait_until(time);
                } else {
                    control_flow.set_wait();
                }
            }

            _ => (),
        }
    });
}
