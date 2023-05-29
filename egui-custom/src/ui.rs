use std::{
    cell::{Ref, RefCell, RefMut},
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
    time::{Duration, Instant},
};

use egui::Context;
use winit::{
    dpi::Size,
    event::WindowEvent,
    event_loop::EventLoopWindowTarget,
    platform::windows::{WindowBuilderExtWindows, WindowExtWindows, HWND},
    window::{Window, WindowBuilder, WindowButtons, WindowId},
};

/// Interface for an Egui ui inside an os window.
pub trait Ui {
    /// Returns the window options for this window.
    fn get_window_options(&self) -> WindowOptions {
        WindowOptions::default()
    }
    /// Runs the ui code.
    fn show(&mut self, ctx: &egui::Context, windower: &mut Windower);
}

/// A reference to a Ui object running inside a os window.
///
/// The Ui object and the associated os window are destroyed and dropped when
/// all handles to it have been dropped.
///
/// If the window is closed by other means, this handle will continue to
/// allow access to the Ui object even if the window itself no longer exists.
pub struct UiHandle<T: Ui + ?Sized> {
    value: Rc<RefCell<UiContainer<T>>>,
}

impl<T: Ui + 'static> UiHandle<T> {
    /// Create a new ui handle for a Ui object.
    pub fn new(ui: T) -> UiHandle<T> {
        UiHandle {
            value: Rc::new(RefCell::new(UiContainer {
                events: Vec::new(),
                ui,
            })),
        }
    }
    /// Transform this handle from a concrete type into its trait object type.
    pub fn to_dyn(self) -> UiHandle<dyn Ui> {
        let dyn_rc: Rc<RefCell<UiContainer<dyn Ui>>> = self.value;
        UiHandle { value: dyn_rc }
    }
}

impl<T: Ui + ?Sized> UiHandle<T> {
    /// Return a clone of this Handle as a weak reference.
    pub fn as_weak(&self) -> WeakUiHandle<T> {
        WeakUiHandle {
            value: Rc::downgrade(&self.value),
        }
    }

    /// Immutably borrow the internal Ui object.
    pub fn borrow_ui(&self) -> Ref<UiContainer<T>> {
        (*self.value).borrow()
    }

    /// Mutably borrow the internal Ui object.
    pub fn borrow_ui_mut(&self) -> RefMut<UiContainer<T>> {
        (*self.value).borrow_mut()
    }
}

impl<T: Ui + ?Sized> Clone for UiHandle<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

// impl<T: Ui + ?Sized> Deref for UiHandle<T> {
//     type Target = RefCell<UiContainer<T>>;

//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }

/// A weak version of a ui handle. This handle will not keep the ui
/// from being dropped.
pub struct WeakUiHandle<T: Ui + ?Sized> {
    value: Weak<RefCell<UiContainer<T>>>,
}

impl<T: Ui + ?Sized> WeakUiHandle<T> {
    /// Upgrade to a UiHandle.
    pub fn upgrade(&self) -> Option<UiHandle<T>> {
        self.value.upgrade().map(|rc| UiHandle { value: rc })
    }
}

/// Allows the creating of windows from inside a egui context.
pub struct Windower<'a> {
    events: &'a mut Vec<UiEvent>,
}

impl<'a> Windower<'a> {
    pub fn new_window<T: Ui + 'static>(&mut self, ui: T) -> UiHandle<T> {
        let ui_handle = UiHandle::new(ui);
        self.events
            .push(UiEvent::CreateWindow(ui_handle.clone().to_dyn()));
        ui_handle
    }
}

/// Wrapps around a specific ui object and collects events that
/// were created for or by the ui.
pub struct UiContainer<T: Ui + ?Sized> {
    events: Vec<UiEvent>,
    ui: T,
}

impl<T: Ui + ?Sized> UiContainer<T> {
    /// Request a redraw for the window.
    pub fn request_redraw(&mut self) {
        self.events.push(UiEvent::RequestRedraw);
    }

    /// Close this window. The underlying Ui will remain accessable
    /// unitl all handles to it have been dropped.
    pub fn close(&mut self) {
        self.events.push(UiEvent::Close);
    }

    /// Show this ui.
    fn show(&mut self, egui_ctx: &Context) {
        let mut windower = Windower {
            events: &mut self.events,
        };
        self.ui.show(egui_ctx, &mut windower);
    }

    /// Return all current events for this ui and clear the list.
    fn take_events(&mut self) -> Vec<UiEvent> {
        let events = self.events.clone();
        self.events.clear();
        events
    }
}

impl<T: Ui + ?Sized> Deref for UiContainer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ui
    }
}

impl<T: Ui + ?Sized> DerefMut for UiContainer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ui
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

/// Events that can be raised on a Ui window.
#[derive(Clone)]
pub(crate) enum UiEvent {
    CreateWindow(UiHandle<dyn Ui>),
    RequestRedraw,
    Close,
}

/// An os window that can display a egui Ui.
pub(crate) struct UiWindow {
    ui: WeakUiHandle<dyn Ui>,
    redraw_time: Option<Instant>,
    modal: Option<WindowId>,
    backend: Backend,
}

impl UiWindow {
    /// Create a new os window backend.
    pub fn new(ui: UiHandle<dyn Ui>, backend: Backend) -> Self {
        let mut ui_window = UiWindow {
            ui: ui.as_weak(),
            redraw_time: None,
            modal: None,
            backend,
        };
        ui_window.run_and_paint();
        ui_window.backend.window.set_visible(true);
        ui_window
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

    /// Run the egui ui on this window.
    pub fn run_and_paint(&mut self) {
        let ui = match self.ui.upgrade() {
            Some(ui) => ui,
            // If the Ui has been dropped then this window should be destroyed aswell.
            None => return,
        };

        let repaint_after = self.backend.run_and_paint(ui);

        if repaint_after.is_zero() {
            // We want to redraw on the next frame so we create a request for right now.
            // This will trigger a `window.request_redraw()` next frame.
            self.set_redraw_time(Instant::now());
        } else if let Some(time) = Instant::now().checked_add(repaint_after) {
            // Trigger a redraw at some time in the future.
            self.set_redraw_time(time);
        }
    }

    /// Return all ui events associated with this window and clear.
    pub fn poll_ui_events(&mut self) -> Vec<UiEvent> {
        match self.ui.upgrade() {
            Some(ui) => ui.borrow_ui_mut().take_events(),
            // If the Ui has been dropped then this window should also be closed.
            None => {
                vec![UiEvent::Close]
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
            Some(size) => window_builder.with_inner_size(size),
            None => window_builder,
        };
        window_builder = match window_options.min_size {
            Some(size) => window_builder.with_min_inner_size(size),
            None => window_builder,
        };
        window_builder = match window_options.max_size {
            Some(size) => window_builder.with_max_inner_size(size),
            None => window_builder,
        };

        let window = window_builder.build(window_target).unwrap();

        let mut painter =
            egui_wgpu::winit::Painter::new(egui_wgpu::WgpuConfiguration::default(), 1, 0, false);
        unsafe {
            pollster::block_on(painter.set_window(Some(&window))).unwrap();
        }

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

    /// Run the egui ui on this window.
    pub fn run_and_paint(&mut self, ui: UiHandle<dyn Ui>) -> Duration {
        // Gather input (mouse, touches, keyboard, screen size, etc):
        let raw_input: egui::RawInput = self.state.take_egui_input(&self.window);

        let egui::FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = self.context.run(raw_input, |egui_ctx| {
            ui.borrow_ui_mut().show(egui_ctx);
        });

        self.state
            .handle_platform_output(&self.window, &self.context, platform_output);

        let clipped_primitives = self.context.tessellate(shapes); // creates triangles to paint
        self.painter.paint_and_update_textures(
            self.state.pixels_per_point(),
            [1.0, 1.0, 1.0, 1.0],
            &clipped_primitives,
            &textures_delta,
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
