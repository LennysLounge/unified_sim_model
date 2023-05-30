use std::{collections::HashMap, env};

use egui::{Context, RichText, Ui};
use egui_custom::dialog::{Dialog, Windower};
use egui_extras::{Column, TableBuilder};
use tracing::info;
use tracing_subscriber::EnvFilter;
use unified_sim_model::{
    adapter::Adapter,
    model::{Entry, EntryId},
};

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

    egui_custom::run_event_loop(App { adapter: None });
}

struct App {
    adapter: Option<Adapter>,
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

            if let Some(adapter) = &self.adapter {
                let model = adapter
                    .model
                    .read()
                    .expect("Should be able to lock for reading");
                if let Some(session) = model.current_session() {
                    display_entries_table(ui, &session.entries);
                }
            }
        });
    }
}

fn display_entries_table(ui: &mut Ui, entries: &HashMap<EntryId, Entry>) {
    let mut entries: Vec<&Entry> = entries.values().collect();
    entries.sort_by(|e1, e2| e1.position.cmp(&e2.position));

    TableBuilder::new(ui)
        .auto_shrink([false, false])
        .resizable(true)
        .striped(true)
        .column(Column::exact(20.0))
        .column(Column::initial(150.0).clip(true).at_least(20.0))
        .column(Column::initial(150.0).clip(true).at_least(20.0))
        .column(Column::exact(20.0))
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Pos").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Team name").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Driver").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.strong("#");
                    },
                );
            });
        })
        .body(|body| {
            body.rows(25.0, entries.len(), |i, mut row| {
                let entry = entries[i];
                row.col(|ui| {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::TopDown),
                        |ui| {
                            ui.add(egui::Label::new(format!("{}", entry.position)).wrap(false));
                        },
                    );
                });
                row.col(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(&entry.team_name).wrap(false));
                    });
                });
                row.col(|ui| {
                    let driver_name = match entry.drivers.get(&entry.current_driver) {
                        Some(driver) => format!("{} {}", driver.first_name, driver.last_name),
                        None => "No driver".to_string(),
                    };
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(driver_name);
                    });
                });
                row.col(|ui| {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.label(format!("#{}", entry.car_number));
                        },
                    );
                });
            });
        });
}
