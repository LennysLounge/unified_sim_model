use std::{cell::RefCell, fmt::Debug, time::Instant};
use tracing::info;
use tree::Tree;
use window::{Backend, Ui};
use winit::{event::WindowEvent, event_loop::EventLoopBuilder, window::WindowId};

use crate::window::{UiEvents, WindowOptions};

mod tree;
pub mod window;

pub enum UserEvent {
    /// Event for adding a new window.
    CreateWindow {
        /// Window id of the parent window.
        src_id: WindowId,
        /// Backend object to use.
        backend: Backend,
    },
    /// Destroy a window.
    DestroyWindow {
        /// Id of the window to destroy.
        id: WindowId,
    },
    /// Request a redraw for a window.
    RequestRedraw(
        /// The if of the window to redraw
        WindowId,
    ),
}

impl Debug for UserEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateWindow { src_id, .. } => f
                .debug_struct("CreateWindow")
                .field("src_id", src_id)
                .finish_non_exhaustive(),
            Self::DestroyWindow { id } => f.debug_struct("DestroyWindow").field("id", id).finish(),
            Self::RequestRedraw(id) => f.debug_tuple("RequestRedraw").field(id).finish(),
        }
    }
}

/// A function that creates a AppWindow.
pub type AppCreator = Box<dyn Fn() -> Box<dyn Ui>>;

/// Run the event loop with a app.
pub fn run_event_loop<T: Ui + Clone + 'static>(window_options: WindowOptions, ui: T) {
    let mut window_tree: Tree<WindowId, RefCell<Backend>> = Tree::new();

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    event_loop.run(move |event, window_target, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                for node in window_tree.values() {
                    node.value.borrow_mut().update_redraw_timer();
                }
            }

            Event::UserEvent(event) => match event {
                // Add window to tree
                UserEvent::CreateWindow { src_id, backend } => {
                    let new_window_id = backend.window_id();
                    window_tree.add_node(new_window_id, RefCell::new(backend));
                    window_tree.add_child_to_parent(new_window_id, src_id);
                }
                UserEvent::DestroyWindow { id: src_id } => {
                    window_tree.remove(src_id);
                }
                UserEvent::RequestRedraw(id) => {
                    window_tree
                        .get(&id)
                        .map(|backend| backend.borrow_mut().set_redraw_time(Instant::now()));
                }
            },

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
                    app_state.borrow_mut().on_window_event(event);
                }
            }

            // Create the apps here.
            Event::Resumed => {
                let app_state = RefCell::new(Backend::new(
                    window_target,
                    &window_options,
                    UiEvents::new_with_handle(ui.clone()),
                ));
                let id = app_state.borrow().window_id();
                window_tree.add_node(id, app_state);
            }

            Event::MainEventsCleared => {
                let mut all_events = Vec::<(WindowId, window::UiEvent)>::new();
                for (window_id, node) in window_tree.iter() {
                    for event in node.value.borrow_mut().poll_ui_events() {
                        all_events.push((window_id.clone(), event));
                    }
                }
                for (src_window_id, event) in all_events {
                    match event {
                        window::UiEvent::CreateWindow(ui_handle) => {
                            info!("Create window event received");
                            let app_state = RefCell::new(Backend::new(
                                window_target,
                                &window_options,
                                ui_handle,
                            ));
                            let id = app_state.borrow().window_id();
                            window_tree.add_node(id, app_state);
                            window_tree.add_child_to_parent(id, src_window_id);
                        }
                    }
                }
            }

            // Redraw the requested window.
            Event::RedrawRequested(window_id) => {
                if let Some(app_state) = window_tree.get(&window_id) {
                    app_state.borrow_mut().run_and_paint();
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
