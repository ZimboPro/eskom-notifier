use eframe::{egui::{Sense, Button}, epaint::Color32};

use crate::{layouts::top::top_bar, traits::Page, ActivePage, StateData};

#[derive(Debug, Default)]
pub struct HomePage {}

impl Page for HomePage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut StateData) {
    top_bar(ui, state, false);
    if !state.ids.is_empty() {
      state.ids.iter().for_each(|area| {
        ui.horizontal(|ui| {
          let b = Button::new(area.details.info.name.clone());
          // let r = ui.label(area.details.info.name.clone());
          // let r = ui.interact(r.rect, r.id, Sense::click());
          // TODO disable button with spinner if getting updated details
          if ui.add(b).clicked() {
            println!("Clicked");
            state.page = ActivePage::AreaDetails(area.id.clone());
          }
          ui.label("-");
          ui.label(area.details.info.region.clone());
        });
      });
    }
    if ui.button("Add Region").clicked() {
      state.page = ActivePage::FindArea;
    }
  }
}
