use eframe::egui::ScrollArea;

use crate::{traits::Page, ActivePage};

#[derive(Debug, Default)]
pub struct AreaDetailsPage {
  details: Option<crate::SelectedAreaInfo>
}

impl Page for AreaDetailsPage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::StateData) {
    if let ActivePage::AreaDetails(id) = &state.page {
      ui.label("Area Details");
      if self.details.is_none() {
        self.details = Some(state.ids.iter().find(|x| x.id == *id).unwrap().clone());
      }
      if let Some(details) = &self.details {
        ui.label(details.details.info.name.as_str());
        ui.label(details.details.info.region.as_str());
        ScrollArea::horizontal().id_source("Stage times").show(ui, |ui| {
          ui.horizontal(|ui| {
            for s in &details.details.events {
                ui.vertical(|ui| {
                  ui.label(s.note.as_str());
                  ui.label(s.start.as_str());
                  ui.label(s.end.as_str());
                });
              }
          });
        });
        ScrollArea::vertical().show(ui, |ui| {
          for stage in &details.details.schedule.days {
              ui.label(stage.name.as_str());
              ui.label(stage.date.as_str());
              ui.horizontal(|ui| {
                for (ind, s) in stage.stages.iter().enumerate() {
                  ui.vertical(|ui| {
                    ui.label(format!("Stage {}", ind + 1));
                    for i in s {
                        ui.label(i.as_str());
                    }
                  });
                }
            });
          }
        });
      }
  } else {
    state.page = ActivePage::Home;
  }
  }
}
