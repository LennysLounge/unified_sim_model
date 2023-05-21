use std::time::Duration;

use tracing::info;

pub struct MyApp {
    pub name: String,
    pub age: u32,
    pub checked: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            checked: false,
        }
    }
}

impl MyApp {
    #[allow(dead_code)]
    pub fn update(&mut self, ctx: &egui::Context) {
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
            if ui.button("repaint now").clicked() {
                info!("repaint now clicked");
                ctx.request_repaint();
            }
            if ui.button("repaint now in 5 secs").clicked() {
                info!("repaint later clicked");
                ctx.request_repaint_after(Duration::from_secs(5));
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
