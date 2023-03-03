use crate::{traits::Page, ActivePage, StateData, layouts::top::top_bar};

#[derive(Debug, Default)]
pub struct HomePage {}

impl Page for HomePage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut StateData) {
    top_bar(ui, state, false);
    ui.label("HomePage");
    if !state.ids.is_empty() {
      state.ids.iter().for_each(|id| {
        ui.label(id.details.info.name.clone());
      });
    }
    if ui.button("Add Region").clicked() {
      state.page = ActivePage::FindArea;
    }
  }
}
