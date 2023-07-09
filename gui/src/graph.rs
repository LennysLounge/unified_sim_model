use std::{
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, RwLock,
    },
    thread::{self},
    time::{Duration, Instant},
};

use egui::plot::{Line, PlotPoints};
use egui_custom::dialog::{Dialog, DialogHandle, Size};
use unified_sim_model::{model::EntryId, Adapter};

struct GraphModel {
    data: Vec<(Duration, f32)>,
}

pub struct Graph {
    pub handle: Option<DialogHandle<Graph>>,
    driver_name: String,
    model: Arc<RwLock<GraphModel>>,
    close_channel: Sender<()>,
}

impl Graph {
    pub fn new(adapter: Adapter, entry_id: EntryId) -> Self {
        let graph_model = Arc::new(RwLock::new(GraphModel { data: Vec::new() }));
        let (tx, rx) = mpsc::channel();
        let thread_model = graph_model.clone();
        let thread_adapter = adapter.clone();
        thread::spawn(move || graph_thread(thread_adapter, thread_model, rx, entry_id));

        let model = adapter
            .model
            .read()
            .expect("Model shouldnt become poisoned");
        let driver_name = model
            .current_session()
            .and_then(|session| session.entries.get(&entry_id))
            .map_or("N/a".to_owned(), |entry| {
                let driver = entry
                    .current_driver
                    .and_then(|driver_id| entry.drivers.get(&driver_id));
                match driver {
                    Some(driver) => format!(
                        "{} {} #{}",
                        driver.first_name, driver.last_name, entry.car_number
                    ),
                    None => "N/a".to_owned(),
                }
            });
        Self {
            handle: None,
            driver_name,
            model: graph_model,
            close_channel: tx,
        }
    }
}

impl Dialog for Graph {
    fn show(&mut self, ctx: &egui::Context, _windower: &mut egui_custom::dialog::Windower) {
        dear_egui::set_theme(ctx, dear_egui::SKY);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Distance driven graph for: ");
                ui.label(&self.driver_name);
            });
            let graph_model = self.model.read().expect("Model shouldnt become poisoned");
            let data: PlotPoints = graph_model
                .data
                .iter()
                .map(|(time, value)| [time.as_secs_f64(), *value as f64])
                .collect();
            let line = Line::new(data);
            egui::plot::Plot::new("my_plot").show(ui, |plot_ui| plot_ui.line(line));
        });
        ctx.request_repaint();
    }

    fn on_close(&mut self) {
        _ = self.close_channel.send(());
    }

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

fn graph_thread(
    adapter: Adapter,
    graph_model: Arc<RwLock<GraphModel>>,
    close_request: Receiver<()>,
    entry_id: EntryId,
) {
    let time_zero = Instant::now();
    while adapter.wait_for_update().is_ok() {
        match close_request.try_recv() {
            Err(e) if e == TryRecvError::Empty => (),
            _ => break,
        }
        let model = adapter
            .model
            .read()
            .expect("Game model shouldnt become poisoned");
        let data = model
            .current_session()
            .and_then(|session| session.entries.get(&entry_id));
        if let Some(entry) = data {
            if !*entry.connected {
                continue;
            }
            let now = Instant::now();

            let mut graph_model = graph_model
                .write()
                .expect("The model shouldnt become poisoded");
            graph_model
                .data
                .push((now - time_zero, *entry.distance_driven));
        }
    }
}
