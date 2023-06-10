#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::Ui;
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

struct MyApp {
    fixed: [bool; 30],
}

impl Default for MyApp {
    fn default() -> Self {
        Self { fixed: [false; 30] }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Fixing columns");
            ui.label("A fixed column will always stay visible in the table.");
            ui.separator();

            self.show_table(ui);
        });
    }
}

impl MyApp {
    fn show_table(&mut self, ui: &mut Ui) {
        let mut table = Table::new();
        for c in self.fixed {
            table = table.column(Column::auto().fixed(c));
        }
        table
            .scroll(true, true)
            .striped(true)
            .column_lines(true)
            .show(ui, |table| {
                // Header
                table.row(Row::new().height(40.0).fixed(true), |row| {
                    for (i, is_fixed) in self.fixed.iter_mut().enumerate() {
                        row.cell(|ui| {
                            ui.vertical(|ui| {
                                ui.strong(format!("Column {i}"));
                                ui.checkbox(is_fixed, "fixed");
                            });
                        });
                    }
                });
                // Body
                for _ in 0..20 {
                    table.row(Row::new().height(20.0), |row| {
                        for i in 0..self.fixed.len() {
                            row.cell(|ui| {
                                ui.label(format!("cell {i}"));
                            });
                        }
                    });
                }
            });
    }
}
