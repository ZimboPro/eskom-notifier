use eframe::egui::Ui;

use crate::StateData;

pub trait Page {
  fn page(&mut self, ui: &mut Ui, state: &mut StateData);
}
