use eframe::egui::Ui;

use crate::{ActivePage, StateData};

pub fn top_bar(ui: &mut Ui, state: &mut StateData, disable_all: bool) {
  ui.horizontal_top(|ui| {
    ui.add_enabled_ui(ActivePage::Home != state.page && !disable_all, |ui| {
      if ui.button("Home").clicked() {
        state.page = ActivePage::Home;
      }
    });
    ui.add_enabled_ui(ActivePage::FindArea != state.page && !disable_all, |ui| {
      if ui.button("Add Region").clicked() {
        state.page = ActivePage::FindArea;
      }
    });
    ui.add_enabled_ui(ActivePage::Settings != state.page && !disable_all, |ui| {
      if ui.button("Settings").clicked() {
        state.page = ActivePage::Settings;
      }
    });
  });
}
