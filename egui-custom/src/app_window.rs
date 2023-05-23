use std::time::Instant;

use winit::{
    event::WindowEvent,
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowId},
};

use crate::WindowProxy;

/// Interface to show an egui context in a native os window.
/// This function is called to run the gui inside a native os window.
pub trait AppWindow {
    fn update(&mut self, ctx: &egui::Context, windower: &mut WindowProxy);
}

/// A function that creates a AppWindow.
pub type AppCreator = Box<dyn Fn() -> Box<dyn AppWindow>>;

/// Bundles together all data needed to draw egui in a native os window.
pub struct AppWindowState {
    window: Window,
    state: egui_winit::State,
    context: egui::Context,
    app: Box<dyn AppWindow>,
    painter: egui_wgpu::winit::Painter,
    redraw_time: Option<Instant>,
}

impl AppWindowState {
    pub fn new_creator(event_loop: &EventLoopWindowTarget<()>, creator: &AppCreator) -> Self {
        AppWindowState::new(event_loop, creator())
    }

    pub fn new(event_loop: &EventLoopWindowTarget<()>, app_window: Box<dyn AppWindow>) -> Self {
        let window = Window::new(event_loop).unwrap();

        let mut painter =
            egui_wgpu::winit::Painter::new(egui_wgpu::WgpuConfiguration::default(), 1, 0, false);
        unsafe {
            pollster::block_on(painter.set_window(Some(&window))).unwrap();
        }

        let mut state = egui_winit::State::new(event_loop);
        state.set_pixels_per_point(window.scale_factor() as f32);
        if let Some(size) = painter.max_texture_side() {
            state.set_max_texture_side(size);
        }

        AppWindowState {
            window,
            state,
            context: egui::Context::default(),
            app: app_window,
            painter,
            redraw_time: None,
        }
    }

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

    pub fn on_window_event(&mut self, event: &WindowEvent, window_id: &WindowId) {
        if window_id == &self.window.id() {
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
    }

    /// Run the egui gui on this window.
    pub fn run_and_paint(
        &mut self,
        _event_loop: &EventLoopWindowTarget<()>,
        window_id: &WindowId,
        window_proxy: &mut WindowProxy,
    ) {
        if window_id == &self.window.id() {
            // Gather input (mouse, touches, keyboard, screen size, etc):
            let raw_input: egui::RawInput = self.state.take_egui_input(&self.window);

            let egui::FullOutput {
                platform_output,
                repaint_after,
                textures_delta,
                shapes,
            } = self.context.run(raw_input, |egui_ctx| {
                self.app.update(egui_ctx, window_proxy);
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
                let now = Instant::now();
                let redraw_time = self.redraw_time.get_or_insert(now);
                if now < *redraw_time {
                    self.redraw_time = Some(now);
                }
            } else if let Some(time) = Instant::now().checked_add(repaint_after) {
                // Trigger a redraw at some time in the future.
                let redraw_time = self.redraw_time.get_or_insert(time);
                if time < *redraw_time {
                    self.redraw_time = Some(time);
                }
            }
        }
    }
}
