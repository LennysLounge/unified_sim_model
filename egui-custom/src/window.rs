use std::{cell::RefCell, rc::Rc, time::Instant};

use tracing::info;
use winit::{
    dpi::Size,
    event::WindowEvent,
    event_loop::EventLoopWindowTarget,
    platform::windows::WindowBuilderExtWindows,
    window::{Window, WindowBuilder, WindowButtons, WindowId},
};

use crate::UserEvent;

/// Interface for an Egui ui inside an os window.
pub trait Ui {
    /// Runs the ui code.
    fn show(&mut self, ctx: &egui::Context, windower: &mut Windower);
}

// A handle to a ui.
type UiHandle<T> = Rc<RefCell<UiEvents<T>>>;

/// Expands a specific Ui implementation with a list of
/// events that were raised by or for that ui.
pub struct UiEvents<T: Ui + ?Sized> {
    pub events: Vec<UiEvent>,
    pub ui: T,
}

impl<T: Ui> UiEvents<T> {
    pub fn new_with_handle(ui: T) -> UiHandle<T> {
        Rc::new(RefCell::new(Self {
            events: Vec::new(),
            ui: ui,
        }))
    }
}

/// A reference to a Ui object running inside a os window.
///
/// The Ui object and the associated os window are destroyed and dropped when
/// all handles to it have been dropped.
///
/// If the window is closed by other means, this handle will continue to
/// allow access to the Ui object even if the window itself no longer exists.
pub type WindowHandle<T> = Rc<WindowHandleDropGuard<T>>;

/// The inner struct to a `WindowHandle`.
/// If this instance is dropped, the referenced window is destroyed aswell.
pub struct WindowHandleDropGuard<T: Ui> {
    pub ui: UiHandle<T>,
}

impl<T: Ui> WindowHandleDropGuard<T> {
    /// Request a redraw for the window.
    pub fn request_redraw(&self) {
        self.ui.borrow_mut().events.push(UiEvent::RequestRedraw);
    }
}

impl<T: Ui> Drop for WindowHandleDropGuard<T> {
    fn drop(&mut self) {
        // TODO create destroy window event.
    }
}

#[derive(Clone)]
pub enum UiEvent {
    CreateWindow(UiHandle<dyn Ui>),
    RequestRedraw,
}

/// Allows the creating of windows.
#[derive(Default)]
pub struct Windower {
    events: Vec<UiEvent>,
}

impl Windower {
    pub fn new_window<T: Ui + 'static>(&mut self, ui: T) -> WindowHandle<T> {
        let ui_handle = UiEvents::new_with_handle(ui);
        info!("Add new window event");
        self.events.push(UiEvent::CreateWindow(ui_handle.clone()));
        WindowHandle::new(WindowHandleDropGuard {
            ui: ui_handle.clone(),
        })
    }
}

/// The backend for a os window.
/// Bundles together all necessary objects to run an egui context in
/// a os window.
pub struct Backend {
    window: Window,
    state: egui_winit::State,
    context: egui::Context,
    ui: UiHandle<dyn Ui>,
    painter: egui_wgpu::winit::Painter,
    redraw_time: Option<Instant>,
}

impl Backend {
    /// Create a new os window backend.
    pub fn new(
        window_target: &EventLoopWindowTarget<UserEvent>,
        window_options: &WindowOptions,
        ui: UiHandle<dyn Ui>,
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

        let mut backend = Backend {
            window,
            state,
            context: egui::Context::default(),
            ui,
            painter,
            redraw_time: None,
        };
        backend.run_and_paint();
        backend.window.set_visible(true);
        backend
    }

    /// Return the window id of the os window.
    pub fn window_id(&self) -> WindowId {
        self.window.id()
    }

    /// Update the windows internal redraw timer and triggers a
    /// redraw if the timer expired.
    pub fn update_redraw_timer(&mut self) {
        if let Some(time) = self.redraw_time {
            if time <= Instant::now() {
                self.window.request_redraw();
                self.redraw_time = None;
            }
        }
    }

    /// Return the current redraw timer.
    pub fn get_redraw_timer(&self) -> Option<Instant> {
        self.redraw_time
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
    pub fn run_and_paint(&mut self) {
        // Gather input (mouse, touches, keyboard, screen size, etc):
        let raw_input: egui::RawInput = self.state.take_egui_input(&self.window);

        let mut window_proxy = Windower::default();

        let egui::FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = self.context.run(raw_input, |egui_ctx| {
            self.ui.borrow_mut().ui.show(egui_ctx, &mut window_proxy);
        });

        self.ui.borrow_mut().events.extend(window_proxy.events);

        self.state
            .handle_platform_output(&self.window, &self.context, platform_output);

        let clipped_primitives = self.context.tessellate(shapes); // creates triangles to paint
        self.painter.paint_and_update_textures(
            self.state.pixels_per_point(),
            [1.0, 1.0, 1.0, 1.0],
            &clipped_primitives,
            &textures_delta,
        );

        if repaint_after.is_zero() {
            // We want to redraw on the next frame so we create a request for right now.
            // This will trigger a `window.request_redraw()` next frame.
            self.set_redraw_time(Instant::now());
        } else if let Some(time) = Instant::now().checked_add(repaint_after) {
            // Trigger a redraw at some time in the future.
            self.set_redraw_time(time);
        }
    }

    pub fn set_redraw_time(&mut self, time: Instant) {
        let redraw_time = self.redraw_time.get_or_insert(time);
        if time < *redraw_time {
            self.redraw_time = Some(time);
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn poll_ui_events(&mut self) -> Vec<UiEvent> {
        let events = self.ui.borrow().events.clone();
        self.ui.borrow_mut().events.clear();
        events
    }
}

/// Options for how a window should be created.
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
        }
    }
}
