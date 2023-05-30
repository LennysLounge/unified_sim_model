use dialog::{Backend, Dialog, DialogEvent, DialogHandle, DialogWindow};
use std::{cell::RefCell, time::Instant};
use tracing::info;
use tree::Tree;
use winit::{
    event::WindowEvent,
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

pub mod dialog;
mod tree;

/// A function that creates a AppWindow.
pub type AppCreator = Box<dyn Fn() -> Box<dyn Dialog>>;

/// A container for a tree of windows.
struct WindowTree {
    tree: Tree<WindowId, RefCell<DialogWindow>>,
}
impl WindowTree {
    /// Create a new window tree.
    fn new() -> Self {
        Self { tree: Tree::new() }
    }

    /// Iterate over all dialog windows in this tree.
    fn dialog_windows(&self) -> impl Iterator<Item = &RefCell<DialogWindow>> + '_ {
        self.tree.values()
    }

    /// Close the window and all its child windows.
    fn close_window(&mut self, window_id: WindowId) {
        // Remove modal from parent window
        if let Some(parent_window) = self
            .tree
            .get_node(&window_id)
            .and_then(|node| node.parent)
            .and_then(|parent_window_id| self.tree.get(&parent_window_id))
        {
            parent_window.borrow_mut().set_modal_to(None);
        }
        // Remove window and its children from the tree.
        self.tree.remove(window_id);
    }

    /// Return `true` if this tree contains no windows.
    fn all_windows_closed(&self) -> bool {
        self.tree.is_empty()
    }

    /// Return the dialog window for a given window id.
    fn get(&self, window_id: WindowId) -> Option<&RefCell<DialogWindow>> {
        self.tree.get(&window_id)
    }

    /// Create a new window.
    fn create_window(
        &mut self,
        window_target: &EventLoopWindowTarget<()>,
        parent_window_id: WindowId,
        dialog_handle: DialogHandle<dyn Dialog>,
    ) {
        let window_options = dialog_handle.borrow_dialog().get_window_options();
        // If this window is modal we need to find the window handle of the parent window.
        let owner = match window_options.modal {
            true => self
                .tree
                .get(&parent_window_id)
                .map(|parent_window| parent_window.borrow_mut().get_window_handle()),
            false => None,
        };

        let backend = Backend::new(window_target, &window_options, owner);
        let dialog_window = RefCell::new(DialogWindow::new(dialog_handle, backend));

        // add window to tree
        let id = dialog_window.borrow().window_id();
        self.tree.add_node(id, dialog_window);
        self.tree.add_child_to_parent(id, parent_window_id);

        // set the parent modal to the child
        if window_options.modal {
            if let Some(parent_node) = self.tree.get(&parent_window_id) {
                parent_node.borrow_mut().set_modal_to(Some(id));
            }
        }
    }

    /// Create a new root window.
    fn create_root(
        &mut self,
        window_target: &EventLoopWindowTarget<()>,
        dialog_handle: DialogHandle<dyn Dialog>,
    ) {
        let backend = Backend::new(
            window_target,
            &dialog_handle.borrow_dialog().get_window_options(),
            None,
        );
        let dialog_window = RefCell::new(DialogWindow::new(dialog_handle, backend));
        let id = dialog_window.borrow().window_id();
        self.tree.add_node(id, dialog_window);
    }

    /// Collect dialog events from all windows and return them as a list
    /// of tuples with the window id that created the event.
    fn collect_dialog_events(&self) -> Vec<(WindowId, DialogEvent)> {
        // Gather all dialog events and the window id that caused them.
        let mut all_events = Vec::<(WindowId, dialog::DialogEvent)>::new();
        for dialog_window in self.tree.values() {
            let id = dialog_window.borrow().window_id();
            for event in dialog_window.borrow_mut().poll_events() {
                all_events.push((id, event));
            }
        }
        all_events
    }
}

/// Run the event loop with a app.
pub fn run_event_loop<T: Dialog + 'static>(dialog: T) {
    let mut window_tree = WindowTree::new();
    let root_dialog = DialogHandle::new(dialog).to_dyn();

    EventLoop::new().run(move |event, window_target, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                for dialog_window in window_tree.dialog_windows() {
                    dialog_window.borrow_mut().update_redraw_timer();
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
                window_tree.create_root(window_target, root_dialog.clone());
            }

            Event::MainEventsCleared => {}

            // Redraw the requested window.
            Event::RedrawRequested(window_id) => {
                if let Some(app_state) = window_tree.get(window_id) {
                    app_state.borrow_mut().run_and_paint();
                }
            }

            // At the end of the event cycle poll the generated dialog events and
            // set the control flow.
            Event::RedrawEventsCleared => {
                // Gather all dialog events and the window id that caused them.
                let dialog_events = window_tree.collect_dialog_events();
                // Handle all dialog events.
                for (src_window_id, event) in dialog_events {
                    match event {
                        dialog::DialogEvent::CreateWindow(dialog_handle) => {
                            window_tree.create_window(window_target, src_window_id, dialog_handle);
                        }
                        dialog::DialogEvent::RequestRedraw => {
                            if let Some(dialog_window) = window_tree.get(src_window_id) {
                                dialog_window.borrow_mut().set_redraw_time(Instant::now());
                            }
                        }
                        dialog::DialogEvent::Close => {
                            window_tree.close_window(src_window_id);
                        }
                    }
                }

                // Set control flow.
                let earliest_redraw = window_tree
                    .dialog_windows()
                    .filter_map(|dialog_window| dialog_window.borrow().get_redraw_timer())
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
