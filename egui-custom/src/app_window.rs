use std::time::Instant;

use winit::{event_loop::EventLoopWindowTarget, window::Window};

pub trait App {
    fn update(&mut self, ctx: &egui::Context, windower: Windower);
}

pub type AppCreator = Box<dyn Fn() -> Box<dyn App>>;

pub struct Windower<'a> {
    event_loop: &'a EventLoopWindowTarget<()>,
    new_apps: &'a mut Vec<AppWindow>,
}

impl<'a> Windower<'a> {
    pub fn window(&mut self, creator: AppCreator) {
        self.new_apps
            .push(AppWindow::new(self.event_loop, &creator));
    }
}

/// An app that is created from a Fn.
struct FnApp {
    ui: Box<dyn Fn(&egui::Context, Windower)>,
}
impl App for FnApp {
    fn update(&mut self, ctx: &egui::Context, windower: Windower) {
        (self.ui)(ctx, windower);
    }
}

/// An application window.
pub struct AppWindow {
    pub window: Window,
    pub state: egui_winit::State,
    pub context: egui::Context,
    pub app: Box<dyn App>,
    pub painter: egui_wgpu::winit::Painter,
    pub redraw_time: Option<Instant>,
}

impl AppWindow {
    pub fn new(event_loop: &EventLoopWindowTarget<()>, creator: &AppCreator) -> Self {
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

        AppWindow {
            window,
            state,
            context: egui::Context::default(),
            app: creator(),
            painter,
            redraw_time: None,
        }
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
    pub fn get_redraw_timer(&self) -> Option<&Instant> {
        return self.redraw_time.as_ref();
    }

    /// Run the egui gui on this window.
    pub fn run(&mut self, event_loop: &EventLoopWindowTarget<()>, new_apps: &mut Vec<AppWindow>) {
        // Gather input (mouse, touches, keyboard, screen size, etc):
        let raw_input: egui::RawInput = self.state.take_egui_input(&self.window);

        let windower = Windower {
            new_apps,
            event_loop,
        };

        let egui::FullOutput {
            platform_output,
            repaint_after,
            textures_delta,
            shapes,
        } = self.context.run(raw_input, |egui_ctx| {
            self.app.update(egui_ctx, windower);
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
