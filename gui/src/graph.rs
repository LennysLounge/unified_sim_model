use egui_custom::dialog::{Dialog, DialogHandle, Size};
use unified_sim_model::Adapter;

pub struct Graph {
    pub adapter: Adapter,
    pub handle: Option<DialogHandle<Graph>>,
}

impl Dialog for Graph {
    fn show(&mut self, ctx: &egui::Context, _windower: &mut egui_custom::dialog::Windower) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("text");
        });
    }

    fn on_close(&mut self) {}

    fn get_window_options(&self) -> egui_custom::dialog::WindowOptions {
        egui_custom::dialog::WindowOptions {
            title: "Graph".to_string(),
            size: Some(Size {
                width: 300,
                height: 200,
            }),
            ..egui_custom::dialog::WindowOptions::default()
        }
    }
}
