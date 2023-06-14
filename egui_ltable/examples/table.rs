#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::{Align, Direction, Layout, Sense};
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
                        .column(
                            Column::auto()
                                .fixed(true)
                                .max_width(400.0)
                                .resizeable(true)
                                .layout(
                                    Layout::centered_and_justified(Direction::LeftToRight)
                                        .with_cross_align(Align::Min),
                                ),
                        )
                        .column(
                            Column::initial(300.0)
                                .min_width(100.0)
                                .max_width(400.0)
                                .resizeable(true),
                        )
                        .column(
                            Column::fill(1.0)
                                .min_width(50.0)
                                //.max_width(200.0)
                                .resizeable(true)
                                .layout(Layout::right_to_left(Align::Min).with_main_wrap(false)),
                        )
                        .scroll(true, true)
                        .striped(true)
                        .column_lines(true)
                        .resize_full_height(false)
                        .show(ui, |table| {
                            let r = table.row(
                                Row::new().height(40.0).fixed(true).sense(Sense::click()),
                                |row| {
                                    let r = row.cell_sense(Sense::click(), |ui| {
                                        ui.strong("Column 1");
                                    });
                                    if let Some(r) = r {
                                        if r.clicked() {
                                            println!("Column 1 clicked");
                                        }
                                    }
                                    row.cell(|ui| {
                                        ui.strong("Column 2");
                                    });
                                    row.cell(|ui| {
                                        ui.strong("Column 3");
                                    });
                                },
                            );
                            if r.clicked() {
                                println!("Header was clicked");
                            }

                            for _ in 0..10 {
                                table.row(Row::new().height(40.0), |row| {
                                    row.cell(|ui| {
                                        ui.label("cell 1");
                                    });
                                    row.cell(|ui| {
                                        ui.label(
                                            "cell 2 is a bit longer and might warp at some point",
                                        );
                                    });
                                    row.cell(|ui| {
                                        ui.label("123.3455");
                                    });
                                });
                            }
                        });
                    ui.label("After the table");
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
