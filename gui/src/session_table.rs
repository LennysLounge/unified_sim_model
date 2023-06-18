use std::{collections::HashMap, sync::RwLockReadGuard};

use egui::{RichText, Sense, Ui};
use egui_custom::dialog::Windower;
use egui_ltable::{Column, Row, Table};
use tracing::info;
use unified_sim_model::{
    model::{Entry, EntryId, Event, Model, SessionId},
    Adapter,
};

use crate::graph::Graph;

pub struct SessionTable {
    session_tab_tree: egui_dock::Tree<SessionTab>,
}

impl SessionTable {
    pub fn new() -> Self {
        SessionTable {
            session_tab_tree: egui_dock::Tree::new(Vec::new()),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        model: &RwLockReadGuard<'_, Model>,
        windower: &mut Windower,
        adapter: &Adapter,
    ) {
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
            .scroll_area_in_tabs(false)
            .show_inside(
                ui,
                &mut TabViewer {
                    model,
                    windower,
                    adapter,
                },
            );
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

    pub fn clear(&mut self) {
        self.session_tab_tree = egui_dock::Tree::new(Vec::new());
    }
}

struct SessionTab {
    session_id: SessionId,
    title: String,
}

struct TabViewer<'a, 'b> {
    model: &'a RwLockReadGuard<'a, Model>,
    windower: &'a mut Windower<'b>,
    adapter: &'a Adapter,
}

impl<'a, 'b> egui_dock::TabViewer for TabViewer<'a, 'b> {
    type Tab = SessionTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(session) = self.model.sessions.get(&tab.session_id) {
            display_entries_table(ui, &session.entries, self.windower, self.adapter);
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title.clone().into()
    }
}

fn display_entries_table(
    ui: &mut Ui,
    entries: &HashMap<EntryId, Entry>,
    windower: &mut Windower,
    adapter: &Adapter,
) {
    let mut entries: Vec<&Entry> = entries.values().collect();
    entries.sort_by(|e1, e2| {
        let is_connected = e2.connected.cmp(&e1.connected);
        let position = e2
            .distance_driven
            .partial_cmp(&e1.distance_driven)
            .unwrap_or(std::cmp::Ordering::Equal);
        is_connected.then(position)
    });

    let center = egui::Layout::centered_and_justified(egui::Direction::LeftToRight);
    let right = egui::Layout::right_to_left(egui::Align::Min);
    Table::new()
        .striped(true)
        .column(Column::exact(25.0).layout(center)) // pit
        .column(Column::exact(30.0).layout(right)) // pos
        .column(Column::exact(30.0).layout(right)) // #
        .column(Column::initial(100.0).resizeable(true).min_width(70.0)) // team
        .column(Column::initial(150.0).resizeable(true).min_width(70.0)) // driver
        .column(Column::initial(75.0).resizeable(true).min_width(50.0)) // car
        .column(Column::exact(70.0).layout(right)) // spline pos
        .column(Column::exact(50.0).layout(right)) // laps
        .column(Column::exact(70.0).layout(right)) // best lap
        .column(Column::exact(70.0).layout(right)) // lap
        .column(Column::exact(70.0).layout(right)) // delta
        .column(Column::exact(70.0).layout(right)) // to leader
        .column(Column::exact(70.0).layout(right)) // stint
        .column(Column::fill(1.0).min_width(0.1))
        .column_lines(true)
        .resize_full_height(false)
        .scroll(true, true)
        .show(ui, |table| {
            // Headers
            table.row(Row::new().height(20.0).fixed(true), |row| {
                row.cell(|_| {});
                row.cell(|ui| {
                    ui.strong("Pos");
                });
                row.cell(|ui| {
                    ui.strong("#");
                });
                row.cell(|ui| {
                    ui.strong("Team name");
                });
                row.cell(|ui| {
                    ui.strong("Driver");
                });
                row.cell(|ui| {
                    ui.strong("Car");
                });
                row.cell(|ui| {
                    ui.strong("Spline pos");
                });
                row.cell(|ui| {
                    ui.strong("Laps");
                });
                row.cell(|ui| {
                    ui.strong("Best lap");
                });
                row.cell(|ui| {
                    ui.strong("Lap");
                });
                row.cell(|ui| {
                    ui.strong("Delta");
                });
                row.cell(|ui| {
                    ui.strong("To leader");
                });
                row.cell(|ui| {
                    ui.strong("Stint");
                });
                row.cell(|_| {});
            });

            // Body
            for entry in entries {
                let response = table.row(
                    Row::new()
                        .height(20.0)
                        .hover_highlight(true)
                        .sense(Sense::click()),
                    |row| {
                        row.cell(|ui| {
                            if entry.in_pits {
                                ui.label("P");
                            }
                        });
                        row.cell(|ui| {
                            let s = if entry.connected {
                                format!("{}", entry.position)
                            } else {
                                "-".to_string()
                            };
                            ui.add(egui::Label::new(s).wrap(false));
                        });
                        row.cell(|ui| {
                            ui.label(format!("{}", entry.car_number));
                        });
                        row.cell(|ui| {
                            ui.add(egui::Label::new(&entry.team_name).wrap(false));
                        });
                        row.cell(|ui| {
                            let driver_name = match entry.drivers.get(&entry.current_driver) {
                                Some(driver) => {
                                    format!("{} {}", driver.first_name, driver.last_name)
                                }
                                None => "No driver".to_string(),
                            };
                            ui.label(driver_name);
                        });
                        row.cell(|ui| {
                            ui.label(entry.car.name);
                        });
                        let r = row.cell_sense(Sense::click(), |ui| {
                            ui.label(format!("{:.3}", entry.distance_driven));
                        });
                        if let Some(response) = r {
                            if response.double_clicked() {
                                info!("Row cell clicked");
                            }
                            response.context_menu(|ui| {
                                if ui.button("Focus").clicked() {
                                    ui.close_menu();
                                }
                                if ui.button("Graph").clicked() {
                                    let graph =
                                        windower.new_window(Graph::new(adapter.clone(), entry.id));
                                    graph.borrow_dialog_mut().handle = Some(graph.clone());
                                    ui.close_menu();
                                }
                            });
                        }
                        row.cell(|ui| {
                            ui.label(format!("{}", entry.lap_count));
                        });
                        row.cell(|ui| {
                            let best_lap = entry
                                .drivers
                                .get(&entry.current_driver)
                                .and_then(|driver| driver.best_lap)
                                .and_then(|lap_index| entry.laps.get(lap_index))
                                .map_or("-".to_string(), |lap| lap.time.format());

                            ui.label(best_lap);
                        });
                        row.cell(|ui| {
                            let mut lap_time = RichText::new(entry.current_lap.time.format());
                            if entry.current_lap.invalid {
                                lap_time = lap_time.color(egui::Color32::RED);
                            }

                            ui.label(lap_time);
                        });
                        row.cell(|ui| {
                            let mut delta = RichText::new(entry.performance_delta.format());
                            if entry.current_lap.invalid {
                                delta = delta.color(egui::Color32::RED);
                            } else if entry.performance_delta.ms < 0 {
                                delta = delta.color(egui::Color32::GREEN);
                            }
                            ui.label(delta);
                        });
                        row.cell(|ui| {
                            ui.label(entry.time_behind_leader.format());
                        });
                        row.cell(|ui| {
                            ui.label(entry.stint_time.format());
                        });
                        row.cell(|_| {});
                    },
                );
                if response.double_clicked() {
                    info!("Row clicked");
                }
                response.context_menu(|ui| {
                    if ui.button("Focus").clicked() {
                        ui.close_menu();
                    }
                });
            }
        });
}
