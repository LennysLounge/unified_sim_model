use std::{cell::RefCell, time::Instant};
use tracing::info;
use tree::Tree;
use window::{Backend, Ui, UiHandle};
use winit::{event::WindowEvent, event_loop::EventLoop, window::WindowId};

use crate::window::WindowOptions;

mod tree;
pub mod window;

/// A function that creates a AppWindow.
pub type AppCreator = Box<dyn Fn() -> Box<dyn Ui>>;

/// Run the event loop with a app.
pub fn run_event_loop<T: Ui + Clone + 'static>(window_options: WindowOptions, ui: T) {
    let mut window_tree: Tree<WindowId, RefCell<Backend>> = Tree::new();
    let ui_handle = UiHandle::new(ui.clone()).to_dyn();

    EventLoop::new().run(move |event, window_target, control_flow| {
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
                    app_state.borrow_mut().on_window_event(event);
                }
            }

            // Create the apps here.
            Event::Resumed => {
                let app_state = RefCell::new(Backend::new(
                    window_target,
                    &window_options,
                    ui_handle.as_weak(),
                ));
                let id = app_state.borrow().window_id();
                window_tree.add_node(id, app_state);
            }

            Event::MainEventsCleared => {}

            // Redraw the requested window.
            Event::RedrawRequested(window_id) => {
                if let Some(app_state) = window_tree.get(&window_id) {
                    app_state.borrow_mut().run_and_paint();
                }
            }

            // At the end of the event cycle poll the generated ui events and
            // set the control flow.
            Event::RedrawEventsCleared => {
                // Gather all ui events and the window id that caused them.
                let mut all_events = Vec::<(WindowId, window::UiEvent)>::new();
                for (window_id, node) in window_tree.iter() {
                    for event in node.value.borrow_mut().poll_ui_events() {
                        all_events.push((window_id.clone(), event));
                    }
                }
                // Handle all ui events.
                for (src_window_id, event) in all_events {
                    match event {
                        window::UiEvent::CreateWindow(ui_handle) => {
                            let app_state = RefCell::new(Backend::new(
                                window_target,
                                &window_options,
                                ui_handle.as_weak(),
                            ));
                            let id = app_state.borrow().window_id();
                            window_tree.add_node(id, app_state);
                            window_tree.add_child_to_parent(id, src_window_id);
                        }
                        window::UiEvent::RequestRedraw => {
                            if let Some(node) = window_tree.get(&src_window_id) {
                                node.borrow_mut().set_redraw_time(Instant::now());
                            }
                        }
                        window::UiEvent::Close => {
                            window_tree.remove(src_window_id);
                        }
                    }
                }

                // Set control flow.
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
