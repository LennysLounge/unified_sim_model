use std::env;

use egui::Context;
use egui_custom::dialog::{Dialog, Size, Windower};

use session_table::SessionTable;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;
use unified_sim_model::{Adapter, AdapterCommand};

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
        dear_egui::set_theme(ctx, dear_egui::SKY);

        // Check adapter state.
        if let Some(ref mut adapter) = self.adapter {
            if adapter.is_finished() {
                if let Some(Err(e)) = adapter.join() {
                    info!("Connection closed: {:?}", e);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let is_adapter_active = self
                .adapter
                .as_ref()
                .is_some_and(|adapter| !adapter.is_finished());
            if is_adapter_active {
                if ui.button("Disconnect").clicked() {
                    self.close_adpater();
                }
            } else {
                if ui.button("Dummy").clicked() {
                    self.adapter = Some(Adapter::new_dummy());
                    self.session_table.clear();
                }
                if ui.button("Connect").clicked() {
                    self.adapter = Some(Adapter::new_acc());
                    self.session_table.clear();
                }
            }
            ui.separator();

            if let Some(ref adapter) = self.adapter {
                self.session_table.show(ui, &adapter.model.read().unwrap());
            }
        });

        // clear adapter events at the end of the frame.
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

    fn on_close(&mut self) {
        self.close_adpater();
    }
}

impl App {
    fn close_adpater(&mut self) {
        if let Some(ref mut adapter) = self.adapter {
            if adapter.is_finished() {
                return;
            }
            adapter.send(AdapterCommand::Close);
            if let Some(Err(e)) = adapter.join() {
                warn!("Connection closed: {:?}", e);
            } else {
                info!("Adapter shut down correctly");
            }
        }
    }
}
