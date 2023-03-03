use std::{
  sync::mpsc::{self, Receiver, Sender},
  thread::{self, JoinHandle},
};

use eframe::epaint::Color32;
use eskom_se_push_api::{
  area_info::{AreaInfoURLBuilder},
  area_search::{AreaSearch, AreaSearchURLBuilder},
  errors::{HttpError},
  Endpoint,
};

use crate::{helpers::map_error, traits::Page, ActivePage, StateData, layouts::top::top_bar};

#[derive(Debug)]
pub struct FindAreaPage {
  search_term: String,
  thread: Option<JoinHandle<Result<AreaSearch, HttpError>>>,
  testing: bool,
  error: Option<String>,
  area_search: Option<AreaSearch>,
  error_adding: Vec<String>,
  rx: Receiver<crate::SelectedAreaInfo>,
  tx: Sender<crate::SelectedAreaInfo>,
  err_rx: Receiver<HttpError>,
  err_tx: Sender<HttpError>,
  count: usize,
}

impl Page for FindAreaPage {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::StateData) {
    top_bar(ui, state, self.count != 0);
    self.check_recievers(state);
    ui.label("Find Area");
    ui.text_edit_singleline(&mut self.search_term);
    ui.add_enabled_ui(
      !self.search_term.trim().is_empty() && self.search_term.len() > 3,
      |ui| {
        if !self.testing && ui.button("Search").clicked() {
          self.error = None;
          self.testing = true;
          let api_key = state.api_key.clone();
          let search_term = self.search_term.clone();
          self.thread = Some(thread::spawn(move || {
            let t = AreaSearchURLBuilder::default()
              .search_term(search_term)
              .build()
              .unwrap();
            t.reqwest(api_key.as_str())
          }));
        } else if self.testing {
          ui.spinner();
        }
      },
    );
    if let Some(err_msg) = &self.error {
      ui.colored_label(Color32::RED, err_msg);
    }
    if let Some(search) = &self.area_search {
      search.areas.iter().for_each(|s| {
        ui.horizontal_top(|ui| {
          ui.label(s.name.as_str());
          ui.label(s.region.as_str());
          if ui.button("Add region").clicked() {
            self.count += 1;
            let tx = self.tx.clone();
            let err_tx = self.err_tx.clone();
            let id = s.id.clone();
            let api_key = state.api_key.clone();
            thread::spawn(move || {
              let details = AreaInfoURLBuilder::default()
                .area_id(id.clone())
                .build()
                .unwrap();
              match details.reqwest(&api_key) {
                Ok(t) => {
                  let _ = tx.send(crate::SelectedAreaInfo { id, details: t });
                }
                Err(e) => {
                  let _ = err_tx.send(e);
                }
              }
            });
          }
        });
      });
    }
    ui.add_enabled_ui(self.count == 0, |ui| {
      if ui.button("Done").clicked() {
        state.page = ActivePage::Home;
      }
    });
    self.check_thread();
  }
}

impl FindAreaPage {
  fn check_thread(&mut self) {
    if self.thread.is_some() {
      if let Some(thr) = self.thread.take() {
        if thr.is_finished() {
          let t = thr.join();
          match t {
            Ok(resp) => match resp {
              Ok(result) => {
                self.area_search = Some(result);
              }
              Err(err) => {
                self.testing = false;
                self.error = map_error(err);
              }
            },
            Err(e) => {
              self.testing = false;
              eprintln!("Error joining thread: {:?}", e);
              self.error = Some(format!("Error joining thread: {:?}", e));
            }
          }
        } else {
          self.thread = Some(thr);
        }
      }
    }
  }

  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel();
    let (err_tx, err_rx) = mpsc::channel();
    Self {
      search_term: Default::default(),
      thread: Default::default(),
      testing: Default::default(),
      error: Default::default(),
      area_search: Default::default(),
      error_adding: Default::default(),
      count: Default::default(),
      rx,
      tx,
      err_rx,
      err_tx,
    }
  }

  fn check_recievers(&mut self, state: &mut StateData) {
    if let Ok(area) = self.rx.try_recv() {
      state.ids.push(area)
    }
    if let Ok(err) = self.err_rx.try_recv() {
      self.error_adding.push(map_error(err).unwrap());
    }
  }
}
