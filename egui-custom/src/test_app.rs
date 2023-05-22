use std::time::Duration;

use tracing::info;

use crate::app_window::{App, Windower};

pub struct TestApp {
    pub name: String,
    pub age: u32,
    pub checked: bool,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            checked: false,
        }
    }
}

impl App for TestApp {
    fn update(&mut self, ctx: &egui::Context, mut windower: Windower) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
                info!("Button clicked, {}", self.age);
            }
            if ui.button("Open a new window").clicked() {
                windower.window(Box::new(|| Box::new(PopUp { value: 12 })));
            }
            ui.checkbox(&mut self.checked, "Update every second");
            if self.checked {
                ctx.request_repaint_after(Duration::from_secs(1));
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            self.age += 1;
        });
    }
}

struct PopUp {
    value: i32,
}

impl App for PopUp {
    fn update(&mut self, ctx: &egui::Context, _windower: Windower) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("I am a new window!");
            if ui
                .button("i am also a button that you can click!")
                .clicked()
            {
                info!("I am also clicked");
                self.value += 1;
            }
            ui.label(format!("Value is: {}", self.value));
        });
    }
}
