use crate::{traits::Page, ActivePage};

#[derive(Debug, Default)]
pub struct UpdatingPage {}

impl Page for UpdatingPage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::StateData) {
    if let ActivePage::Updating(message) = &state.page {
      ui.label("Updating");
      ui.label(message);
    } else {
      state.page = ActivePage::Home;
    }
  }
}
