mod app;

use app::MyApp;
use std::{env, time::Instant};
use tracing::{info, Level};
use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

struct App {
    window: Window,
    state: egui_winit::State,
    context: egui::Context,
    app: MyApp,
    painter: egui_wgpu::winit::Painter,
    redraw_time: Option<Instant>,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "");

    env_logger::init();

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_thread_names(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set global subscriber");

    // List of currently active apps.
    let mut apps = Vec::<App>::new();

    EventLoop::new().run(move |event, event_loop, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                // Issue redraw requests for windows that requested it
                let now = Instant::now();
                for app in apps.iter_mut() {
                    if let Some(time) = app.redraw_time {
                        if time <= now {
                            app.window.request_redraw();
                            app.redraw_time = None;
                        }
                    }
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Close requested");
                control_flow.set_exit();
                #[allow(clippy::needless_return)]
                return;
            }

            // Pass window events to the apps.
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                for app in apps.iter_mut() {
                    if window_id != app.window.id() {
                        continue;
                    }
                    if let WindowEvent::Resized(size) = event {
                        if size.width > 0 && size.height > 0 {
                            app.painter.on_window_resized(size.width, size.height);
                        }
                    }

                    let response = app.state.on_event(&app.context, event);
                    if response.repaint {
                        app.window.request_redraw();
                    }
                }
            }

            // Create the apps here.
            Event::Resumed => {
                for _ in 0..10 {
                    let window = Window::new(event_loop).unwrap();

                    let mut painter = egui_wgpu::winit::Painter::new(
                        egui_wgpu::WgpuConfiguration::default(),
                        1,
                        0,
                        false,
                    );
                    unsafe {
                        pollster::block_on(painter.set_window(Some(&window))).unwrap();
                    }

                    let mut state = egui_winit::State::new(event_loop);
                    state.set_pixels_per_point(window.scale_factor() as f32);
                    if let Some(size) = painter.max_texture_side() {
                        state.set_max_texture_side(size);
                    }

                    apps.push(App {
                        window,
                        state,
                        context: egui::Context::default(),
                        app: MyApp::default(),
                        painter,
                        redraw_time: None,
                    });
                }
            }

            Event::RedrawRequested(window_id) => {
                // App should only be rendered when it needs to.
                for app in apps.iter_mut() {
                    if app.window.id() != window_id {
                        continue;
                    }

                    // Gather input (mouse, touches, keyboard, screen size, etc):
                    let raw_input: egui::RawInput = app.state.take_egui_input(&app.window);

                    let egui::FullOutput {
                        platform_output,
                        repaint_after,
                        textures_delta,
                        shapes,
                    } = app.context.run(raw_input, |egui_ctx| {
                        app.app.update(egui_ctx);
                    });

                    app.state
                        .handle_platform_output(&app.window, &app.context, platform_output);

                    let clipped_primitives = app.context.tessellate(shapes); // creates triangles to paint
                    app.painter.paint_and_update_textures(
                        app.state.pixels_per_point(),
                        [1.0, 1.0, 1.0, 1.0],
                        &clipped_primitives,
                        &textures_delta,
                    );

                    if repaint_after.is_zero() {
                        // We want to redraw on the next frame so we create a request for right now.
                        // This will trigger a `window.request_redraw()` next frame.
                        let now = Instant::now();
                        let redraw_time = app.redraw_time.get_or_insert(now);
                        if now < *redraw_time {
                            app.redraw_time = Some(now);
                        }
                    } else if let Some(time) = Instant::now().checked_add(repaint_after) {
                        // Trigger a redraw at some time in the future.
                        let redraw_time = app.redraw_time.get_or_insert(time);
                        if time < *redraw_time {
                            app.redraw_time = Some(time);
                        }
                    }
                }
            }

            // At the end of the event cycle set the control flow.
            Event::RedrawEventsCleared => {
                let mut earliest_redraw = None;
                for app in apps.iter() {
                    if let Some(time) = app.redraw_time {
                        let redraw_time = earliest_redraw.get_or_insert(time);
                        if time < *redraw_time {
                            earliest_redraw = Some(time);
                        }
                    }
                }

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
