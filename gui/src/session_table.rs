use std::collections::HashMap;

use egui::{RichText, Sense, Ui};
use egui_custom::dialog::Windower;
use egui_ltable::{Column, Row, Table};
use unified_sim_model::{
    model::{Entry, EntryId, Model, Session},
    Adapter, AdapterCommand,
};

use crate::{graph::Graph, tab_panel::TabPanel};

pub fn show_session_tabs(ui: &mut Ui, model: &Model, windower: &mut Windower, adapter: &Adapter) {
    let mut session_tabs = TabPanel::new(ui);
    for (session_id, session) in model.sessions.iter() {
        session_tabs.add_tab(*session_id, format!("{:?}", *session.session_type));
    }
    session_tabs.show(|session_id, ui| {
        let Some(session) = model.sessions.get(session_id) else {return;};

        #[derive(Clone, PartialEq, Eq)]
        enum SessionTabs {
            Livetiming,
            SessionInfo,
            Camera,
        }
        TabPanel::new(ui)
            .with_tab(SessionTabs::Livetiming, "Livetiming")
            .with_tab(SessionTabs::SessionInfo, "Session info")
            .with_tab(SessionTabs::Camera, "Camera")
            .show(|id, ui| match id {
                SessionTabs::Livetiming => {
                    display_entries_table(ui, &session.entries, windower, adapter)
                }
                SessionTabs::SessionInfo => display_session_info(ui, session),
                SessionTabs::Camera => display_cameras(ui, model, adapter),
            });
    });
}

fn display_cameras(ui: &mut Ui, model: &Model, adapter: &Adapter) {
    for camera in model.available_cameras.iter() {
        if ui.button(format!("{:?}", camera)).clicked() {
            adapter.send(AdapterCommand::ChangeCamera(camera.clone()));
        }
    }
}

fn display_session_info(ui: &mut Ui, session: &Session) {
    egui::Grid::new("session info grid").show(ui, |ui| {
        ui.label("Track:");
        ui.label(session.track_name.as_ref());
        ui.label("Length:");
        ui.label(format!("{}m", session.track_length));
        ui.end_row();

        ui.label("Type:");
        ui.label(format!("{:?}", *session.session_type));
        ui.label("Phase:");
        ui.label(format!("{:?}", *session.phase));
        ui.end_row();

        ui.label("Time:");
        ui.label(session.session_time.format());
        ui.label("Remaining:");
        ui.label(session.time_remaining.format());
        ui.end_row();

        ui.label("Day:");
        ui.label(format!("{:?}", *session.day));
        ui.label("Time of day:");
        ui.label(session.time_of_day.format());
        ui.end_row();

        ui.label("Ambient temp:");
        ui.label(format!("{}", session.ambient_temp));
        ui.label("Track temp:");
        ui.label(format!("{}", session.track_temp));
        ui.end_row();

        ui.label("Best lap:");
        if let Some(best_lap) = session.best_lap.as_ref() {
            ui.label(best_lap.time.format());
        } else {
            ui.label("-");
        }
        ui.end_row();

        if let Some(data) = session.game_data.as_acc() {
            ui.label("Acc data:");
            ui.end_row();

            ui.label("Event index:");
            ui.label(format!("{}", data.event_index));
            ui.label("Session index:");
            ui.label(format!("{}", data.session_index));
            ui.end_row();

            ui.label("Camera set:");
            ui.label(&data.camera_set);
            ui.label("Camera:");
            ui.label(&data.camera);
            ui.label("Hud page:");
            ui.label(&data.hud_page);
            ui.end_row();

            ui.label("Cloud level:");
            ui.label(format!("{}", data.cloud_level));
            ui.label("Rain:");
            ui.label(format!("{}", data.rain_level));
            ui.label("Wetness:");
            ui.label(format!("{}", data.wetness));
            ui.end_row();
        }
    });
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

    let focus_on_car = |entry_id| {
        adapter.send(AdapterCommand::FocusOnCar(entry_id));
    };

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
                    ui.strong("Last lap");
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
                        .highlight(entry.focused)
                        .sense(Sense::click()),
                    |row| {
                        row.cell(|ui| {
                            if *entry.in_pits {
                                ui.label("P");
                            }
                        });
                        row.cell(|ui| {
                            let s = if *entry.connected {
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
                            ui.add(egui::Label::new(entry.team_name.as_ref()).wrap(false));
                        });
                        row.cell(|ui| {
                            let driver = entry.drivers.get(&entry.current_driver);
                            let driver_name = match driver {
                                Some(driver) => {
                                    format!("{} {}", driver.first_name, driver.last_name)
                                }
                                None => "No driver".to_string(),
                            };
                            ui.label(driver_name);
                        });
                        row.cell(|ui| {
                            ui.label(entry.car.name());
                        });
                        let r = row.cell_sense(Sense::click(), |ui| {
                            ui.label(format!("{:.3}", entry.distance_driven));
                        });
                        if let Some(response) = r {
                            if response.double_clicked() {
                                focus_on_car(entry.id);
                            }
                            response.context_menu(|ui| {
                                if ui.button("Focus").clicked() {
                                    ui.close_menu();
                                    focus_on_car(entry.id);
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
                                .best_lap
                                .as_ref()
                                .as_ref()
                                .map_or("-".to_string(), |lap| lap.time.format());
                            ui.label(best_lap);
                        });
                        row.cell(|ui| {
                            let last_lap = entry
                                .laps
                                .last()
                                .map_or("-".to_string(), |lap| lap.time.format());
                            ui.label(last_lap);
                        });
                        row.cell(|ui| {
                            let mut lap_time = RichText::new(entry.current_lap.time.format());
                            if *entry.current_lap.invalid {
                                lap_time = lap_time.color(egui::Color32::RED);
                            }

                            ui.label(lap_time);
                        });
                        row.cell(|ui| {
                            let mut delta = RichText::new(entry.performance_delta.format());
                            if *entry.current_lap.invalid {
                                delta = delta.color(egui::Color32::RED);
                            } else if entry.performance_delta.ms < 0.0 {
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
                    focus_on_car(entry.id);
                }
                response.context_menu(|ui| {
                    if ui.button("Focus").clicked() {
                        ui.close_menu();
                        focus_on_car(entry.id);
                    }
                });
            }
        });
}
