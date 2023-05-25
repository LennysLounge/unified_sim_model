use std::{cell::RefCell, time::Instant};
use tracing::info;
use tree::Tree;
use window::{Backend, Ui, UiHandle};
use winit::{
    event::WindowEvent,
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

use crate::window::WindowOptions;

mod tree;
pub mod window;

/// A function that creates a AppWindow.
pub type AppCreator = Box<dyn Fn() -> Box<dyn Ui>>;

/// A container for a tree of windows.
struct WindowTree {
    tree: Tree<WindowId, RefCell<Backend>>,
}
impl WindowTree {
    /// Create a new window tree.
    fn new() -> Self {
        Self { tree: Tree::new() }
    }

    /// Iterate over all window backends in this tree.
    fn backends(&self) -> impl Iterator<Item = &RefCell<Backend>> + '_ {
        self.tree.values()
    }

    /// Close the window and all its child windows.
    fn close_window(&mut self, window_id: WindowId) {
        // Remove modal from parent window
        if let Some(node) = self.tree.get_node(&window_id) {
            if let Some(parent_window_id) = node.parent {
                if let Some(parent_backend) = self.tree.get(&parent_window_id) {
                    parent_backend.borrow_mut().set_modal(None);
                }
            }
        }

        self.tree.remove(window_id);
    }

    /// Return `true` if this tree contains no windows.
    fn all_windows_closed(&self) -> bool {
        self.tree.is_empty()
    }

    /// Return the backend for a given window.
    fn get(&self, window_id: WindowId) -> Option<&RefCell<Backend>> {
        self.tree.get(&window_id)
    }

    /// Create a new window.
    fn create_window(
        &mut self,
        window_target: &EventLoopWindowTarget<()>,
        parent_window_id: WindowId,
        window_options: &WindowOptions,
        ui_handle: UiHandle<dyn Ui>,
    ) {
        let owner = match window_options.modal {
            true => self
                .tree
                .get(&parent_window_id)
                .map(|parent_backend| parent_backend.borrow_mut().get_window_handle()),
            false => None,
        };
        let app_state = RefCell::new(Backend::new(
            window_target,
            window_options,
            ui_handle.as_weak(),
            owner,
        ));
        let id = app_state.borrow().window_id();
        self.tree.add_node(id, app_state);
        self.tree.add_child_to_parent(id, parent_window_id);
        if window_options.modal {
            if let Some(parent_node) = self.tree.get(&parent_window_id) {
                parent_node.borrow_mut().set_modal(Some(id));
            }
        }
    }

    /// Create a new root window.
    fn create_root(
        &mut self,
        window_target: &EventLoopWindowTarget<()>,
        window_options: &WindowOptions,
        ui_handle: UiHandle<dyn Ui>,
    ) {
        let app_state = RefCell::new(Backend::new(
            window_target,
            window_options,
            ui_handle.as_weak(),
            None,
        ));
        let id = app_state.borrow().window_id();
        self.tree.add_node(id, app_state);
    }
}

/// Run the event loop with a app.
pub fn run_event_loop<T: Ui + Clone + 'static>(window_options: WindowOptions, ui: T) {
    let mut window_tree = WindowTree::new();
    let ui_handle = UiHandle::new(ui).to_dyn();

    EventLoop::new().run(move |event, window_target, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                for backend in window_tree.backends() {
                    backend.borrow_mut().update_redraw_timer();
                }
            }

            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } => {
                window_tree.close_window(window_id);

                if window_tree.all_windows_closed() {
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
                if let Some(app_state) = window_tree.get(window_id) {
                    app_state.borrow_mut().on_window_event(event);
                }
            }

            // Create the apps here.
            Event::Resumed => {
                window_tree.create_root(window_target, &window_options, ui_handle.clone());
            }

            Event::MainEventsCleared => {}

            // Redraw the requested window.
            Event::RedrawRequested(window_id) => {
                if let Some(app_state) = window_tree.get(window_id) {
                    app_state.borrow_mut().run_and_paint();
                }
            }

            // At the end of the event cycle poll the generated ui events and
            // set the control flow.
            Event::RedrawEventsCleared => {
                // Gather all ui events and the window id that caused them.
                let mut all_events = Vec::<(WindowId, window::UiEvent)>::new();
                for backend in window_tree.backends() {
                    let id = backend.borrow().window_id();
                    for event in backend.borrow_mut().poll_ui_events() {
                        all_events.push((id, event));
                    }
                }
                // Handle all ui events.
                for (src_window_id, event) in all_events {
                    match event {
                        window::UiEvent::CreateWindow(window_options, ui_handle) => {
                            window_tree.create_window(
                                window_target,
                                src_window_id,
                                &window_options,
                                ui_handle,
                            );
                        }
                        window::UiEvent::RequestRedraw => {
                            if let Some(node) = window_tree.get(src_window_id) {
                                node.borrow_mut().set_redraw_time(Instant::now());
                            }
                        }
                        window::UiEvent::Close => {
                            window_tree.close_window(src_window_id);
                        }
                    }
                }

                // Set control flow.
                let earliest_redraw = window_tree
                    .backends()
                    .filter_map(|node| node.borrow().get_redraw_timer())
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
