#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
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
                    Table::new()
                        .column(Column::auto().fixed(true).min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().fixed(true).min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().fixed(true).min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .column(Column::auto().min_width(50.0).max_width(200.0))
                        .scroll(true, true)
                        .striped(true)
                        .show(ui, |table| {
                            table.row(Row::new().height(40.0).fixed(true), |row| {
                                for i in 0..30 {
                                    row.cell(|ui| {
                                        ui.strong(format!("Column {i}"));
                                    });
                                }
                            });
                            for _ in 0..10 {
                                table.row(Row::new().height(40.0), |row| {
                                    for i in 0..30 {
                                        row.cell(|ui| {
                                            ui.label(format!("cell {i}"));
                                        });
                                    }
                                });
                            }
                        });
                });

            // egui::ScrollArea::both().show(ui, |ui| {
            //     TableBuilder::new(ui, &mut self.table)
            //         .column(Column::new().initial_width(150.0))
            //         .column(Column::new().initial_width(150.0))
            //         .column(Column::new().initial_width(150.0))
            //         .header(|headers| {
            //             headers.height(20.0);
            //             headers.next_header(|ui| {
            //                 ui.label("column 1");
            //             });
            //             headers.next_header(|ui| {
            //                 ui.label("column 2");
            //             });
            //             headers.next_header(|ui| {
            //                 ui.label("column 3");
            //             });
            //         })
            //         .body(|body| {
            //             for _ in 0..10 {
            //                 body.row(|row| {
            //                     row.height(20.0);
            //                     row.next_cell(|ui| ui.label("Cell 1"));
            //                     row.next_cell(|ui| ui.label("Cell 2"));
            //                     row.next_cell(|ui| ui.label("Cell 3"));
            //                 });
            //             }
            //         });
            // });
            // ui.label("Table done");
        });
    }
}
