use std::{collections::HashMap, env};

use egui::{Align, Context, InnerResponse, RichText, Ui};
use egui_custom::dialog::{Dialog, Size, Windower};
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
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        display_entries_table(ui, &session.entries);
                    });
                }
            }
        });

        if self.adapter.is_some() {
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

fn display_entries_table(ui: &mut Ui, entries: &HashMap<EntryId, Entry>) {
    let mut entries: Vec<&Entry> = entries.values().collect();
    entries.sort_by(|e1, e2| {
        let is_connected = e2.connected.cmp(&e1.connected);
        let position = e1.position.cmp(&e2.position);
        is_connected.then(position)
    });

    fn center<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
            add_contents,
        )
    }
    fn right<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
                .with_main_align(Align::Max),
            add_contents,
        )
    }

    TableBuilder::new(ui)
        .auto_shrink([false, false])
        .resizable(true)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(Align::Center))
        .column(Column::exact(20.0))
        .column(Column::exact(20.0))
        .column(Column::initial(150.0).clip(true).at_least(20.0))
        .column(Column::initial(150.0).clip(true).at_least(20.0))
        .column(Column::exact(40.0))
        .column(Column::initial(100.0).clip(true).at_least(20.0))
        .column(Column::initial(70.0))
        .column(Column::initial(50.0))
        .column(Column::initial(50.0))
        .column(Column::initial(50.0))
        .column(Column::initial(50.0))
        .column(Column::initial(50.0))
        .column(Column::initial(70.0))
        .header(20.0, |mut header| {
            header.col(|_| {});
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
                center(ui, |ui| {
                    ui.strong("#");
                });
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Car").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Spline Pos").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Laps").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Lap").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Best lap").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Delta").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("to leader").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Stint").strong()).wrap(false));
            });
        })
        .body(|body| {
            body.rows(20.0, entries.len(), |i, mut row| {
                let entry = entries[i];
                row.col(|ui| {
                    center(ui, |ui| {
                        if entry.in_pits {
                            ui.label("P");
                        }
                    });
                });
                row.col(|ui| {
                    center(ui, |ui| {
                        let s = if entry.connected {
                            format!("{}", entry.position)
                        } else {
                            "-".to_string()
                        };
                        ui.add(egui::Label::new(s).wrap(false));
                    });
                });
                row.col(|ui| {
                    ui.add(egui::Label::new(&entry.team_name).wrap(false));
                });
                row.col(|ui| {
                    let driver_name = match entry.drivers.get(&entry.current_driver) {
                        Some(driver) => format!("{} {}", driver.first_name, driver.last_name),
                        None => "No driver".to_string(),
                    };
                    ui.label(driver_name);
                });
                row.col(|ui| {
                    center(ui, |ui| {
                        ui.label(format!("#{}", entry.car_number));
                    });
                });
                row.col(|ui| {
                    ui.label(entry.car.name);
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(format!("{:.3}", entry.spline_pos));
                    });
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(format!("{}", entry.lap_count));
                    });
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(entry.current_lap.time.format());
                    });
                });
                row.col(|ui| {
                    let best_lap = entry
                        .drivers
                        .get(&entry.current_driver)
                        .and_then(|driver| driver.best_lap)
                        .and_then(|lap_index| entry.laps.get(lap_index))
                        .map_or("-".to_string(), |lap| lap.time.format());

                    right(ui, |ui| {
                        ui.label(best_lap);
                    });
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(entry.performance_delta.format());
                    });
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(entry.time_behind_leader.format());
                    });
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(entry.stint_time.format());
                    });
                });
            });
        });
}
