use crate::traits::Page;

#[derive(Debug, Default)]
pub struct SettingsPage {}

impl Page for SettingsPage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::StateData) {
    ui.label("Settings");
  }
}
