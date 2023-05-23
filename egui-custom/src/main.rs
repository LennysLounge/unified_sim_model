mod app_window;
mod test_app;
mod tree;

use app_window::{AppCreator, AppWindow, AppWindowState};
use std::{cell::RefCell, env};
use test_app::TestApp;
use tracing::{info, Level};
use tree::Tree;
use winit::{event::WindowEvent, event_loop::EventLoop, window::WindowId};

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

pub struct WindowRequest {
    src_window: WindowId,
    app_window: Box<dyn AppWindow>,
}

/// A Proxy to interact with different windows.
pub struct WindowProxy<'a> {
    id: WindowId,
    new_window_requests: &'a mut Vec<WindowRequest>,
}

impl<'a> WindowProxy<'a> {
    fn new_window(&mut self, app: impl AppWindow + 'static) {
        self.new_window_requests.push(WindowRequest {
            src_window: self.id,
            app_window: Box::new(app),
        });
    }
}

fn run_event_loop(creator: AppCreator) {
    let mut window_tree: Tree<WindowId, RefCell<AppWindowState>> = Tree::new();

    EventLoop::new().run(move |event, event_loop, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                for node in window_tree.values() {
                    node.value.borrow_mut().update_redraw_timer();
                }
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } => {
                window_tree.remove(window_id);

                if window_tree.is_empty() {
                    info!("All windows closed. Quitting");
                    control_flow.set_exit();
                    #[allow(clippy::needless_return)]
                    return;
                }
            }

            // Pass window events to the apps.
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                if let Some(app_state) = window_tree.get(&window_id) {
                    app_state.borrow_mut().on_window_event(event, &window_id);
                }
            }

            // Create the apps here.
            Event::Resumed => {
                let app_state = RefCell::new(AppWindowState::new_creator(event_loop, &creator));
                let id = app_state.borrow().window_id();
                window_tree.add_node(id, app_state);
            }

            Event::RedrawRequested(window_id) => {
                let mut window_requests = Vec::<WindowRequest>::new();

                if let Some(app_state) = window_tree.get(&window_id) {
                    let mut window_proxy = WindowProxy {
                        id: window_id,
                        new_window_requests: &mut window_requests,
                    };
                    app_state
                        .borrow_mut()
                        .run_and_paint(event_loop, &window_id, &mut window_proxy);
                }
                for request in window_requests {
                    let app_state =
                        RefCell::new(AppWindowState::new(event_loop, request.app_window));
                    let new_window_id = app_state.borrow().window_id();
                    window_tree.add_node(new_window_id, app_state);

                    // Add window to tree
                    window_tree.add_child_to_parent(new_window_id, request.src_window);
                }
            }

            // At the end of the event cycle set the control flow.
            Event::RedrawEventsCleared => {
                let earliest_redraw = window_tree
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
