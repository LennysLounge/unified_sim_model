use std::{cell::RefCell, ops::Deref, rc::Rc, time::Instant};

use winit::{
    event::WindowEvent,
    event_loop::{EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowId},
};

use crate::UserEvent;

/// Interface for an Egui ui inside an os window.
pub trait Ui {
    /// Runs the ui code.
    fn show(&mut self, ctx: &egui::Context, windower: &mut Windower);
}

/// Allows the creating of windows.
pub struct Windower<'a> {
    id: WindowId,
    event_loop: EventLoopProxy<UserEvent>,
    window_target: &'a EventLoopWindowTarget<UserEvent>,
}

impl<'a> Windower<'a> {
    /// Create a new window.
    /// Returns a window handle to the new window.
    pub fn new_window<T: Ui + 'static>(&mut self, app: T) -> WindowHandle<T> {
        let rc = Rc::new(RefCell::new(app));
        let state = Backend::new(self.window_target, self.event_loop.clone(), rc.clone());
        let new_window_id = state.window_id();
        self.event_loop
            .send_event(UserEvent::CreateWindow {
                src_id: self.id,
                backend: state,
            })
            .unwrap();
        WindowHandle::new(WindowHandleInner {
            ui: rc,
            id: new_window_id,
            event_loop: self.event_loop.clone(),
        })
    }
}

/// A reference to a Ui object running inside a os window.
///
/// The Ui object and the associated os window are destroyed and dropped when
/// all handles to it have been dropped.
///
/// If the window is closed by other means, this handle will continue to
/// allow access to the Ui object even if the window itself no longer exists.
pub type WindowHandle<T> = Rc<WindowHandleInner<T>>;

/// The inner struct to a `WindowHandle`.
/// If this instance is dropped, the referenced window is destroyed aswell.
pub struct WindowHandleInner<T> {
    pub ui: Rc<RefCell<T>>,
    id: WindowId,
    event_loop: EventLoopProxy<UserEvent>,
}

impl<T> WindowHandleInner<T> {
    /// Request a redraw for this window.
    pub fn request_redraw(&self) {
        self.event_loop
            .send_event(UserEvent::RequestRedraw(self.id))
            .unwrap();
    }
}

impl<T> Drop for WindowHandleInner<T> {
    fn drop(&mut self) {
        self.event_loop
            .send_event(UserEvent::DestroyWindow { id: self.id })
            .unwrap();
    }
}

impl<T> Deref for WindowHandleInner<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &(*self.ui)
    }
}

/// The backend for a os window.
/// Bundles together all necessary objects to run an egui context in
/// a os window.
pub struct Backend {
    window: Window,
    state: egui_winit::State,
    context: egui::Context,
    app: Rc<RefCell<dyn Ui>>,
    painter: egui_wgpu::winit::Painter,
    redraw_time: Option<Instant>,
    event_loop_proxy: EventLoopProxy<UserEvent>,
}

impl Backend {
    /// Create a new os window backend.
    pub fn new(
        window_target: &EventLoopWindowTarget<UserEvent>,
        event_loop_proxy: EventLoopProxy<UserEvent>,
        app_window: Rc<RefCell<dyn Ui>>,
    ) -> Self {
        let window = Window::new(window_target).unwrap();

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
            context: egui::Context::default(),
            app: app_window,
            painter,
            redraw_time: None,
            event_loop_proxy,
        }
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
    pub fn run_and_paint(&mut self, window_target: &EventLoopWindowTarget<UserEvent>) {
        // Gather input (mouse, touches, keyboard, screen size, etc):
        let raw_input: egui::RawInput = self.state.take_egui_input(&self.window);

        let mut window_proxy = Windower {
            id: self.window_id(),
            event_loop: self.event_loop_proxy.clone(),
            window_target,
        };

        let egui::FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = self.context.run(raw_input, |egui_ctx| {
            self.app.borrow_mut().show(egui_ctx, &mut window_proxy);
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
}
