mod app_window;
mod test_app;

use app_window::{AppCreator, AppWindow};
use std::env;
use test_app::TestApp;
use tracing::{info, Level};
use winit::{event::WindowEvent, event_loop::EventLoop};

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

    run_event_loop(Box::new(|| Box::new(TestApp::default())));
}

fn run_event_loop(creator: AppCreator) {
    // List of currently active apps.
    let mut apps = Vec::<AppWindow>::new();
    let mut new_apps = Vec::<AppWindow>::new();

    EventLoop::new().run(move |event, event_loop, control_flow| {
        use winit::event::Event;
        match event {
            Event::NewEvents(_) => {
                // Issue redraw requests for windows that requested it
                for app in apps.iter_mut() {
                    app.update_redraw_timer();
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
                apps.push(AppWindow::new(event_loop, &creator));
            }

            Event::RedrawRequested(window_id) => {
                // App should only be rendered when it needs to.
                for app in apps.iter_mut() {
                    if app.window.id() != window_id {
                        continue;
                    }
                    app.run(event_loop, &mut new_apps);
                }
                while let Some(app) = new_apps.pop() {
                    apps.push(app);
                }
            }

            // At the end of the event cycle set the control flow.
            Event::RedrawEventsCleared => {
                let earliest_redraw = apps.iter().filter_map(|app| app.get_redraw_timer()).min();

                if let Some(time) = earliest_redraw {
                    control_flow.set_wait_until(*time);
                } else {
                    control_flow.set_wait();
                }
            }

            _ => (),
        }
    });
}
