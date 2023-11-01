#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Sense, Ui};
use egui_ltable::{Column, Row, Table};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        //initial_window_size: Some(egui::vec2(320.0, 240.0)),
        initial_window_size: Some(egui::vec2(960.0, 720.0)),
        default_theme: eframe::Theme::Dark,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::group(ui.style())
                .inner_margin(egui::Margin::same(100.0))
                .show(ui, |ui| {
                    show_table(ui);
                });
        });
    }
}

fn show_table(ui: &mut Ui) {
    Table::new()
        .column(Column::fill(1.0).min_width(100.0).fixed(true))
        .column(Column::fill(3.0).min_width(300.0))
        .column(Column::fill(3.0).min_width(300.0))
        .scroll(true, true)
        .striped(true)
        .show(ui, |table| {
            let r = table.row(
                Row::new()
                    .height(40.0)
                    .fixed(true)
                    .hover_highlight(true)
                    .sense(Sense::click()),
                |row| {
                    row.cell(|ui| {
                        ui.strong("Column 1");
                    });
                    row.cell(|ui| {
                        ui.strong("Column 2");
                    });
                    row.cell(|ui| {
                        ui.strong("Column 3");
                    });
                },
            );
            if r.clicked() {
                println!("Header clicked");
            }
            for i in 0..10 {
                let r = table.row(
                    Row::new()
                        .height(40.0)
                        .hover_highlight(true)
                        .highlight(i == 5)
                        .sense(Sense::click()),
                    |row| {
                        let r = row.cell_sense(Sense::click(), |ui| {
                            ui.label("cell 1");
                        });
                        if r.is_some_and(|r| r.clicked()) {
                            println!("cell 1 row {i} was clicked");
                        }
                        row.cell(|ui| {
                            ui.label("cell 2 is a bit longer and might warp at some point");
                        });
                        row.cell(|ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(false),
                                |ui| {
                                    ui.label("cell 3 is a bit longer and might warp at some point");
                                },
                            );
                        });
                    },
                );
                if r.clicked() {
                    println!("row {i} as clicked");
                }
            }
            table.row(Row::new().height(40.0).fixed(true), |row| {
                row.cell(|ui| {
                    ui.strong("Column 1");
                });
                row.cell(|ui| {
                    ui.strong("Column 2");
                });
                row.cell(|ui| {
                    ui.strong("Column 3");
                });
            });
            for _ in 0..10 {
                table.row(Row::new().height(40.0), |row| {
                    row.cell(|ui| {
                        ui.label("cell 1");
                    });
                    row.cell(|ui| {
                        ui.label("cell 2 is a bit longer and might warp at some point");
                    });
                    row.cell(|ui| {
                        ui.with_layout(
                            egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(false),
                            |ui| {
                                ui.label("cell 3 is a bit longer and might warp at some point");
                            },
                        );
                    });
                });
            }
        });
}
