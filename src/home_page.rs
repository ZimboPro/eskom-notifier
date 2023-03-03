use eframe::egui::Sense;

use crate::{traits::Page, ActivePage, StateData, layouts::top::top_bar};

#[derive(Debug, Default)]
pub struct HomePage {}

impl Page for HomePage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut StateData) {
    top_bar(ui, state, false);
    ui.label("HomePage");
    if !state.ids.is_empty() {
      state.ids.iter().for_each(|area| {
        let r = ui.label(area.details.info.name.clone());
        let r = ui.interact(r.rect, r.id, Sense::click());
        if r.clicked() {
          println!("Clicked");
          state.page = ActivePage::AreaDetails(area.id.clone());
        }
      });
    }
    if ui.button("Add Region").clicked() {
      state.page = ActivePage::FindArea;
    }
  }
}
