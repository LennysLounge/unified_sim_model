use std::env;

use egui::Context;
use egui_custom::dialog::{Dialog, Size, Windower};

use session_table::SessionTable;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use unified_sim_model::adapter::Adapter;

mod session_table;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "info,gui::testing=trace");
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_thread_names(true)
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("Should be the only time the default is set");

    egui_custom::run_event_loop(App::new());
}

struct App {
    adapter: Option<Adapter>,
    session_table: SessionTable,
}

impl App {
    fn new() -> Self {
        Self {
            adapter: None,
            session_table: SessionTable::new(),
        }
    }
}

impl Dialog for App {
    fn show(&mut self, ctx: &Context, _windower: &mut Windower) {
        // Check adapter state.
        if let Some(ref mut adapter) = self.adapter {
            if let Some(Err(e)) = adapter.take_result() {
                info!("Connection closed: {:?}", e);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref adapter) = self.adapter {
                if adapter.is_finished() {
                    if ui.button("Reconnect").clicked() {
                        info!("TODO: Reconnect adapter");
                        // TODO: Reconnect adapter.
                    }
                } else {
                    if ui.button("Disconnect").clicked() {
                        info!("TODO: Disconnect adapter");
                        // TODO: disconnect adapter.
                    }
                }
            } else {
                if ui.button("Dummy").clicked() {
                    self.adapter = Some(Adapter::new_dummy());
                }
                if ui.button("Connect").clicked() {
                    self.adapter = Some(Adapter::new_acc());
                }
            }

            ui.separator();

            if let Some(ref adapter) = self.adapter {
                self.session_table.show(ui, &adapter.model.read().unwrap());
            }
        });

        if let Some(ref mut adapter) = self.adapter {
            if let Err(e) = adapter.clear_events() {
                error!("Cannot clear events. Model is poisoned: {}", e);
            }
            ctx.request_repaint();
        }
    }

    fn get_window_options(&self) -> egui_custom::dialog::WindowOptions {
        egui_custom::dialog::WindowOptions {
            size: Some(Size {
                width: 960,
                height: 720,
            }),
            ..Default::default()
        }
    }
}
