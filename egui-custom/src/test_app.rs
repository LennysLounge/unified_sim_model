use std::time::Duration;

use egui_custom::ui::{Ui, UiHandle, WindowOptions, Windower};
use tracing::info;
use winit::{
    dpi::{PhysicalSize, Size},
    window::WindowButtons,
};

#[derive(Clone)]
pub struct TestApp {
    pub name: String,
    pub age: u32,
    pub checked: bool,
    popup: Option<UiHandle<PopUp>>,
    popups: Vec<UiHandle<PopUp>>,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            checked: false,
            popup: None,
            popups: Vec::new(),
        }
    }
}

impl Ui for TestApp {
    fn get_window_options(&self) -> WindowOptions {
        WindowOptions {
            title: "Test window".to_string(),
            size: Some(Size::Physical(PhysicalSize {
                width: 340,
                height: 260,
            })),
            ..Default::default()
        }
    }

    fn show(&mut self, ctx: &egui::Context, windower: &mut Windower) {
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
                let new_window = windower.new_window(PopUp { value: 12 });
                if let Some(old_window) = self.popup.take() {
                    self.popups.push(old_window);
                }
                self.popup = Some(new_window);
            }
            ui.checkbox(&mut self.checked, "Update every second");
            if self.checked {
                ctx.request_repaint_after(Duration::from_secs(1));
            }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            if let Some(popup) = &self.popup {
                if ui.button("Increase value").clicked() {
                    popup.borrow_ui_mut().increase();
                    popup.borrow_ui_mut().request_redraw();
                }
                ui.label(format!("The popup has value: {}", popup.borrow_ui().value));
                if ui.button("Close window").clicked() {
                    popup.borrow_ui_mut().close();
                }
                ui.separator();
            }

            if !self.popups.is_empty()
                && ui
                    .button("Close old windows by dropping their handles")
                    .clicked()
            {
                self.popups.clear();
            }
            self.age += 1;
        });
    }
}

struct PopUp {
    value: i32,
}

impl PopUp {
    fn increase(&mut self) {
        self.value += 1;
    }
}

impl Ui for PopUp {
    fn show(&mut self, ctx: &egui::Context, _windower: &mut Windower) {
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

    fn get_window_options(&self) -> WindowOptions {
        WindowOptions {
            enabled_buttons: WindowButtons::CLOSE,
            resizeable: false,
            size: Some(Size::Physical(PhysicalSize {
                width: 220,
                height: 80,
            })),
            modal: true,
            ..Default::default()
        }
    }
}
