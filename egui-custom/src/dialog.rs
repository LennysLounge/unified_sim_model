use std::{
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
    time::{Duration, Instant},
};

use egui::Context;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoopWindowTarget,
    platform::windows::{WindowBuilderExtWindows, WindowExtWindows, HWND},
    window::{Window, WindowBuilder, WindowButtons, WindowId},
};

/// Interface for an Egui dialog displayed in an os window.
pub trait Dialog {
    /// Returns the window options for this window.
    fn get_window_options(&self) -> WindowOptions {
        WindowOptions::default()
    }
    /// Runs the dialog code.
    fn show(&mut self, ctx: &egui::Context, windower: &mut Windower);
}

/// A reference to a Dialog object running inside a os window.
///
/// The Dialog object and the associated os window are destroyed and dropped when
/// all handles to it have been dropped.
///
/// If the window is closed by other means, this handle will continue to
/// allow access to the Dialog object even if the window itself no longer exists.
pub struct DialogHandle<T: Dialog + ?Sized> {
    value: Rc<RefCell<DialogContainer<T>>>,
}

impl<T: Dialog + 'static> DialogHandle<T> {
    /// Create a new dialog handle for a dialog object.
    pub fn new(dialog: T) -> DialogHandle<T> {
        DialogHandle {
            value: Rc::new(RefCell::new(DialogContainer {
                events: Vec::new(),
                dialog,
            })),
        }
    }
    /// Transform this handle from a concrete type into its trait object type.
    pub fn to_dyn(self) -> DialogHandle<dyn Dialog> {
        let dyn_rc: Rc<RefCell<DialogContainer<dyn Dialog>>> = self.value;
        DialogHandle { value: dyn_rc }
    }
}

impl<T: Dialog + ?Sized> DialogHandle<T> {
    /// Return a clone of this Handle as a weak reference.
    pub fn as_weak(&self) -> WeakDialogHandle<T> {
        WeakDialogHandle {
            value: Rc::downgrade(&self.value),
        }
    }

    /// Immutably borrow the internal dialog object.
    pub fn borrow_dialog(&self) -> Ref<DialogContainer<T>> {
        (*self.value).borrow()
    }

    /// Mutably borrow the internal dialog object.
    pub fn borrow_dialog_mut(&self) -> RefMut<DialogContainer<T>> {
        (*self.value).borrow_mut()
    }
}

impl<T: Dialog + ?Sized> Clone for DialogHandle<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

/// A weak version of a dialog handle. This handle will not keep the dialog
/// from being dropped.
pub struct WeakDialogHandle<T: Dialog + ?Sized> {
    value: Weak<RefCell<DialogContainer<T>>>,
}

impl<T: Dialog + ?Sized> WeakDialogHandle<T> {
    /// Upgrade to a DialogHandle.
    pub fn upgrade(&self) -> Option<DialogHandle<T>> {
        self.value.upgrade().map(|rc| DialogHandle { value: rc })
    }
}

/// Allows the creating of windows from inside a egui context.
pub struct Windower<'a> {
    events: &'a mut Vec<DialogEvent>,
}

impl<'a> Windower<'a> {
    pub fn new_window<T: Dialog + 'static>(&mut self, dialog: T) -> DialogHandle<T> {
        let dialog_handle = DialogHandle::new(dialog);
        self.events
            .push(DialogEvent::CreateWindow(dialog_handle.clone().to_dyn()));
        dialog_handle
    }
}

/// Wrapps around a specific dialog object and collects events that
/// were created for or by the dialog.
pub struct DialogContainer<T: Dialog + ?Sized> {
    events: Vec<DialogEvent>,
    dialog: T,
}

impl<T: Dialog + ?Sized> DialogContainer<T> {
    /// Request a redraw for the window.
    pub fn request_redraw(&mut self) {
        self.events.push(DialogEvent::RequestRedraw);
    }

    /// Close this window. The underlying dialog will remain accessable
    /// unitl all handles to it have been dropped.
    pub fn close(&mut self) {
        self.events.push(DialogEvent::Close);
    }

    /// Show this dialog.
    fn show(&mut self, egui_ctx: &Context) {
        let mut windower = Windower {
            events: &mut self.events,
        };
        self.dialog.show(egui_ctx, &mut windower);
    }

    /// Return all current events for this dialog and clear the list.
    fn take_events(&mut self) -> Vec<DialogEvent> {
        let events = self.events.clone();
        self.events.clear();
        events
    }
}

impl<T: Dialog + ?Sized> Deref for DialogContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.dialog
    }
}

impl<T: Dialog + ?Sized> DerefMut for DialogContainer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dialog
    }
}

/// Options for how a window should be created
#[derive(Clone)]
pub struct WindowOptions {
    /// The title of the window.
    pub title: String,

    /// Whether the window will be initially focused or not.
    /// Default true.
    pub active: bool,

    /// Sets whether the window should have a border, a title bar, etc.
    /// Default true.
    pub decorated: bool,

    /// Sets the enabled window buttons.
    /// Default `WindowButtons::all`.
    pub enabled_buttons: WindowButtons,

    /// Request that the window is maximized upon creation.
    /// Default false.
    pub maximised: bool,

    /// Requests the window to be created with this size.
    pub size: Option<Size>,

    /// The minimum allowed size of window.
    pub min_size: Option<Size>,

    /// The maximum allowed size of the window.
    pub max_size: Option<Size>,

    /// Whether the window is resizeable or not.
    /// Default true.
    pub resizeable: bool,

    /// True if this window should behave like a modal window.
    ///
    /// The parent window will become disabled for all input and move focus
    /// to this window. The parent window can only be interacted with again once
    /// this window closes.
    /// Default false.
    pub modal: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Egui window".to_string(),
            active: true,
            decorated: true,
            enabled_buttons: WindowButtons::all(),
            maximised: false,
            size: None,
            min_size: None,
            max_size: None,
            resizeable: true,
            modal: false,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Events that can be raised on a dialog window.
#[derive(Clone)]
pub(crate) enum DialogEvent {
    CreateWindow(DialogHandle<dyn Dialog>),
    RequestRedraw,
    Close,
}

/// An os window that can display a dialog.
pub(crate) struct DialogWindow {
    dialog: WeakDialogHandle<dyn Dialog>,
    redraw_time: Option<Instant>,
    modal: Option<WindowId>,
    backend: Backend,
}

impl DialogWindow {
    /// Create a new os window backend.
    pub fn new(dialog: DialogHandle<dyn Dialog>, backend: Backend) -> Self {
        let mut dialog_window = DialogWindow {
            dialog: dialog.as_weak(),
            redraw_time: None,
            modal: None,
            backend,
        };
        dialog_window.run_and_paint();
        dialog_window.backend.window.set_visible(true);
        dialog_window
    }

    /// Update the windows internal redraw timer and triggers a
    /// redraw if the timer expired.
    pub fn update_redraw_timer(&mut self) {
        if let Some(time) = self.redraw_time {
            if time <= Instant::now() {
                self.backend.window.request_redraw();
                self.redraw_time = None;
            }
        }
    }

    /// Return the current redraw instant.
    pub fn get_redraw_timer(&self) -> Option<Instant> {
        self.redraw_time
    }

    /// Set the instant when the window should be redrawn.
    pub fn set_redraw_time(&mut self, time: Instant) {
        let redraw_time = self.redraw_time.get_or_insert(time);
        if time < *redraw_time {
            self.redraw_time = Some(time);
        }
    }

    /// Handle window events that are ment for this window.
    pub fn on_window_event(&mut self, event: &WindowEvent) {
        self.backend.on_window_event(event);
    }

    /// Run the dialog on this window.
    pub fn run_and_paint(&mut self) {
        let dialog = match self.dialog.upgrade() {
            Some(dialog) => dialog,
            // If the dialog has been dropped then this window should be destroyed aswell.
            None => return,
        };

        let repaint_after = self.backend.run_and_paint(dialog);

        if repaint_after.is_zero() {
            // We want to redraw on the next frame so we create a request for right now.
            // This will trigger a `window.request_redraw()` next frame.
            self.set_redraw_time(Instant::now());
        } else if let Some(time) = Instant::now().checked_add(repaint_after) {
            // Trigger a redraw at some time in the future.
            self.set_redraw_time(time);
        }
    }

    /// Return all dialog events associated with this window and clear.
    pub fn poll_events(&mut self) -> Vec<DialogEvent> {
        match self.dialog.upgrade() {
            Some(handle) => handle.borrow_dialog_mut().take_events(),
            // If the dialog has been dropped then this window should also be closed.
            None => {
                vec![DialogEvent::Close]
            }
        }
    }

    pub fn get_window_handle(&self) -> HWND {
        self.backend.window.hwnd()
    }

    /// Return the window id of the os window.
    pub fn window_id(&self) -> WindowId {
        self.backend.window.id()
    }

    /// Set this window to be modal to another window.
    ///
    /// If given `Some(WindowId)`, this window will be disabled and receive no window events
    /// and give focus to the modal window.
    /// When the child window closes, the parent window (this window) must
    /// be returned to `None`. Otherwise the window will stay disabled.
    ///
    /// When given `None` the window is enabled and receives window events again.
    pub fn set_modal_to(&mut self, modal: Option<WindowId>) {
        self.modal = modal;
        self.backend.window.set_enable(modal.is_none());
    }
}

pub(crate) struct Backend {
    window: Window,
    state: egui_winit::State,
    painter: egui_wgpu::winit::Painter,
    context: egui::Context,
}
impl Backend {
    pub fn new(
        window_target: &EventLoopWindowTarget<()>,
        window_options: &WindowOptions,
        owner: Option<HWND>,
    ) -> Self {
        let mut window_builder = WindowBuilder::new()
            .with_title(window_options.title.clone())
            .with_active(window_options.active)
            .with_decorations(window_options.decorated)
            .with_enabled_buttons(window_options.enabled_buttons)
            .with_maximized(window_options.maximised)
            .with_resizable(window_options.resizeable)
            .with_drag_and_drop(true)
            .with_visible(false);

        window_builder = match owner {
            Some(owner) => window_builder.with_owner_window(owner),
            None => window_builder,
        };

        window_builder = match window_options.size {
            Some(ref size) => {
                window_builder.with_inner_size(winit::dpi::Size::Physical(PhysicalSize {
                    width: size.width,
                    height: size.height,
                }))
            }
            None => window_builder,
        };
        window_builder = match window_options.min_size {
            Some(ref size) => {
                window_builder.with_min_inner_size(winit::dpi::Size::Physical(PhysicalSize {
                    width: size.width,
                    height: size.height,
                }))
            }
            None => window_builder,
        };
        window_builder = match window_options.max_size {
            Some(ref size) => {
                window_builder.with_max_inner_size(winit::dpi::Size::Physical(PhysicalSize {
                    width: size.width,
                    height: size.height,
                }))
            }
            None => window_builder,
        };

        let window = window_builder.build(window_target).unwrap();

        let mut painter =
            egui_wgpu::winit::Painter::new(egui_wgpu::WgpuConfiguration::default(), 1, None, false);
        pollster::block_on(painter.set_window(Some(&window))).unwrap();

        let mut state = egui_winit::State::new(window_target);
        state.set_pixels_per_point(window.scale_factor() as f32);
        if let Some(size) = painter.max_texture_side() {
            state.set_max_texture_side(size);
        }

        Backend {
            window,
            state,
            painter,
            context: egui::Context::default(),
        }
    }

    /// Handle window events that are ment for this window.
    pub fn on_window_event(&mut self, event: &WindowEvent) {
        if let WindowEvent::Resized(size) = event {
            if size.width > 0 && size.height > 0 {
                self.painter.on_window_resized(size.width, size.height);
            }
        }

        let response = self.state.on_event(&self.context, event);
        if response.repaint {
            self.window.request_redraw();
        }
    }

    /// Run the egui dialog on this window.
    pub fn run_and_paint(&mut self, dialog: DialogHandle<dyn Dialog>) -> Duration {
        // Gather input (mouse, touches, keyboard, screen size, etc):
        let raw_input: egui::RawInput = self.state.take_egui_input(&self.window);

        let egui::FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = self.context.run(raw_input, |egui_ctx| {
            dialog.borrow_dialog_mut().show(egui_ctx);
        });

        self.state
            .handle_platform_output(&self.window, &self.context, platform_output);

        let clipped_primitives = self.context.tessellate(shapes); // creates triangles to paint
        self.painter.paint_and_update_textures(
            self.state.pixels_per_point(),
            [1.0, 1.0, 1.0, 1.0],
            &clipped_primitives,
            &textures_delta,
            false,
        );

        // if repaint_after.is_zero() {
        //     // We want to redraw on the next frame so we create a request for right now.
        //     // This will trigger a `window.request_redraw()` next frame.
        //     self.set_redraw_time(Instant::now());
        // } else if let Some(time) = Instant::now().checked_add(repaint_after) {
        //     // Trigger a redraw at some time in the future.
        //     self.set_redraw_time(time);
        // }
        repaint_after
    }
}
