use egui::{util::id_type_map::SerializableAny, Id, Ui, WidgetText};

pub struct TabPanel<'a, T: Clone + Send + SerializableAny + PartialEq> {
    state_id: Id,
    ui: &'a mut Ui,
    selected_tab: Option<T>,
    tabs: Vec<(T, WidgetText)>,
}

impl<'a, T: Clone + Send + SerializableAny + PartialEq> TabPanel<'a, T> {
    pub fn new(ui: &'a mut Ui) -> Self {
        let state_id = ui.id().with("_tab_panel");
        let selected_tab = ui
            .data_mut(|d| d.get_persisted::<Option<T>>(state_id))
            .flatten();
        TabPanel {
            state_id,
            ui,
            selected_tab,
            tabs: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn with_tab(mut self, id: T, title: impl Into<WidgetText>) -> Self {
        self.tabs.push((id, title.into()));
        self
    }

    pub fn add_tab(&mut self, id: T, title: impl Into<WidgetText>) {
        self.tabs.push((id, title.into()));
    }

    pub fn show(mut self, add_content: impl FnOnce(&T, &mut Ui)) {
        if let Some(first_tab) = self.tabs.get(0) {
            // If no tab is selected the first tab should be selected.
            if self.selected_tab.is_none() {
                self.selected_tab = Some(first_tab.0.clone());
            }
        }

        let mut selected_tab_visible = false;
        self.ui.horizontal(|ui| {
            for tab in self.tabs.iter() {
                let is_selected = self
                    .selected_tab
                    .as_ref()
                    .is_some_and(|value| *value == tab.0);
                if is_selected {
                    selected_tab_visible = true;
                }
                if ui.selectable_label(is_selected, tab.1.clone()).clicked() {
                    self.selected_tab = Some(tab.0.clone());
                };
            }
        });

        if !selected_tab_visible {
            // The selected tab is not visible so we set the first tab visible.
            if let Some(first_tab) = self.tabs.get(0) {
                self.selected_tab = Some(first_tab.0.clone());
            }
        }

        self.ui
            .allocate_space(egui::vec2(0.0, -self.ui.spacing().item_spacing.y * 2.0));
        self.ui.separator();
        if let Some(tab_id) = self.selected_tab.as_ref() {
            add_content(tab_id, self.ui);
        }

        self.ui
            .data_mut(|d| d.insert_persisted(self.state_id, self.selected_tab));
    }
}
