use crate::{traits::Page, layouts::top::top_bar};

#[derive(Debug, Default)]
pub struct SettingsPage {}

impl Page for SettingsPage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::StateData) {
    top_bar(ui, state, false);
    ui.label("Settings");
  }
}
