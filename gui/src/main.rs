use std::env;

use egui::Context;
use egui_custom::dialog::{Dialog, Size, Windower};

use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;
use unified_sim_model::{Adapter, AdapterCommand};

mod graph;
mod session_table;
mod tab_panel;

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
}

impl App {
    fn new() -> Self {
        Self { adapter: None }
    }
}

impl Dialog for App {
    fn show(&mut self, ctx: &Context, windower: &mut Windower) {
        dear_egui::set_theme(ctx, dear_egui::SKY);

        // Check adapter state.
        if let Some(ref mut adapter) = self.adapter {
            if adapter.is_finished() {
                if let Some(Err(e)) = adapter.join() {
                    info!("Connection closed: {}", e);
                }
            }
        }

        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Connection", |ui| {
                    let is_adapter_active = self
                        .adapter
                        .as_ref()
                        .is_some_and(|adapter| !adapter.is_finished());
                    if is_adapter_active {
                        if ui.button("Disconnect").clicked() {
                            self.close_adpater();
                            ui.close_menu();
                        }
                    } else {
                        if ui.button("Dummy").clicked() {
                            self.adapter = Some(Adapter::new_dummy());
                            ui.close_menu();
                        }
                        if ui.button("ACC").clicked() {
                            self.adapter = Some(Adapter::new_acc());
                            ui.close_menu();
                        }
                        if ui.button("iRacing").clicked() {
                            self.adapter = Some(Adapter::new_iracing());
                            ui.close_menu();
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let Some(adapter) = self.adapter.as_ref() else {return};
            let Ok(model) = adapter.model.read() else {return};

            ui.label(format!("Event name: {}", model.event_name));
            ui.label(format!("Active Camera: {:?}", *model.active_camera));
            //self.session_table.show(ui, &model, windower, adapter);
            session_table::show_session_tabs(ui, &model, windower, adapter);
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
