use std::{collections::HashMap, sync::RwLockReadGuard};

use egui::{Align, InnerResponse, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use unified_sim_model::model::{Entry, EntryId, Event, Model, SessionId};

pub struct SessionTable {
    session_tab_tree: egui_dock::Tree<SessionTab>,
}

impl SessionTable {
    pub fn new() -> Self {
        SessionTable {
            session_tab_tree: egui_dock::Tree::new(Vec::new()),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, model: &RwLockReadGuard<'_, Model>) {
        self.update_session_tabs(model);

        let mut style = egui_dock::Style::from_egui(ui.style().as_ref());
        style.tabs.rounding = egui::Rounding {
            nw: 5.0,
            ne: 5.0,
            sw: 0.0,
            se: 0.0,
        };
        egui_dock::DockArea::new(&mut self.session_tab_tree)
            .draggable_tabs(false)
            .show_close_buttons(false)
            .style(style)
            .show_inside(ui, &mut TabViewer { model });
    }

    fn update_session_tabs(&mut self, model: &RwLockReadGuard<'_, Model>) {
        for event in model.events.iter() {
            if let Event::SessionChanged(session_id) = event {
                let title = format!(
                    "{:?}",
                    model
                        .sessions
                        .get(session_id)
                        .expect("Session should be availabe after a session change event")
                        .session_type
                );
                self.session_tab_tree.push_to_first_leaf(SessionTab {
                    session_id: session_id.clone(),
                    title,
                });
            }
        }
    }
}

struct SessionTab {
    session_id: SessionId,
    title: String,
}

struct TabViewer<'a> {
    model: &'a RwLockReadGuard<'a, Model>,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = SessionTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(session) = self.model.sessions.get(&tab.session_id) {
            display_entries_table(ui, &session.entries);
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title.clone().into()
    }
}

fn display_entries_table(ui: &mut Ui, entries: &HashMap<EntryId, Entry>) {
    let mut entries: Vec<&Entry> = entries.values().collect();
    entries.sort_by(|e1, e2| {
        let is_connected = e2.connected.cmp(&e1.connected);
        let position = e2
            .distance_driven
            .partial_cmp(&e1.distance_driven)
            .unwrap_or(std::cmp::Ordering::Equal);
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
                ui.add(egui::Label::new(RichText::new("Best lap").strong()).wrap(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Lap").strong()).wrap(false));
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
                        ui.label(format!("{:.3}", entry.distance_driven));
                    });
                });
                row.col(|ui| {
                    right(ui, |ui| {
                        ui.label(format!("{}", entry.lap_count));
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
                    let mut lap_time = RichText::new(entry.current_lap.time.format());
                    if entry.current_lap.invalid {
                        lap_time = lap_time.color(egui::Color32::RED);
                    }

                    right(ui, |ui| {
                        ui.label(lap_time);
                    });
                });
                row.col(|ui| {
                    let mut delta = RichText::new(entry.performance_delta.format());
                    if entry.current_lap.invalid {
                        delta = delta.color(egui::Color32::RED);
                    }
                    right(ui, |ui| {
                        ui.label(delta);
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
