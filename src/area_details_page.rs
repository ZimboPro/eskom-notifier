use crate::traits::Page;

#[derive(Debug, Default)]
pub struct AreaDetailsPage {}

impl Page for AreaDetailsPage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::StateData) {
    ui.label("Area Details");
  }
}
