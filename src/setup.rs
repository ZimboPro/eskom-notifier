use std::thread::{self, JoinHandle};

use crate::helpers::map_error;
use crate::traits::Page;
use crate::{ActivePage, StateData};
use eframe::epaint::Color32;
use eskom_se_push_api::allowance::{AllowanceCheck, AllowanceCheckURLBuilder};
use eskom_se_push_api::errors::HttpError;
use eskom_se_push_api::Endpoint;

#[derive(Debug, Default)]
pub struct Setup {
  testing: bool,
  api_key: String,
  thread: Option<JoinHandle<Result<AllowanceCheck, HttpError>>>,
  error: Option<String>,
}

impl Page for Setup {
  fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut StateData) {
    ui.label("Welcome to the unofficial Eskom-Se-Push Notification Desktop App");
    ui.label("This app will notify you before load shedding starts in your area.");
    ui.label("However, there will be some limitations as follows:");
    ui.label("-> The notifications will not be live. It will check the status every 30 min at the very least.");
    ui.label("-> You will require an API key from Eskom-se-Push. The link is provided below.");
    ui.hyperlink_to("Eskom-se-Push API", "https://eskomsepush.gumroad.com/l/api");

    ui.separator();
    ui.add_enabled_ui(!self.testing, |ui| {
      ui.label("Enter your API key here:");
      ui.text_edit_singleline(&mut self.api_key);
    });
    ui.add_enabled_ui(!self.api_key.is_empty(), |ui| {
      if !self.testing && ui.button("Test").clicked() {
        self.error = None;
        self.testing = true;
        let api_key = self.api_key.clone();
        self.thread = Some(thread::spawn(move || {
          let t = AllowanceCheckURLBuilder::default().build().unwrap();
          t.reqwest(api_key.as_str())
        }));
      } else if self.testing {
        ui.spinner();
      }
    });
    if let Some(err_msg) = &self.error {
      ui.colored_label(Color32::RED, err_msg);
    }
    self.check_thread(state);
  }
}

impl Setup {
  fn check_thread(&mut self, state: &mut StateData) {
    if self.thread.is_some() {
      if let Some(thr) = self.thread.take() {
        if thr.is_finished() {
          let t = thr.join();
          match t {
            Ok(resp) => match resp {
              Ok(_result) => {
                state.page = ActivePage::Home;
                state.api_key = self.api_key.clone();
                state.update_cache();
                self.api_key = String::new();
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
}
