use std::thread::{self, JoinHandle};

use crate::{ActivePage, StateData};
use crate::traits::Page;
use eframe::epaint::Color32;
use eskom_se_push_api::allowance::{AllowanceCheckURLBuilder, AllowanceCheck};
use eskom_se_push_api::errors::{HttpError, APIError};
use eskom_se_push_api::{Endpoint};

#[derive(Debug, Default)]
pub struct Setup { 
    testing: bool,
    api_key: String,
    thread: Option<JoinHandle<Result<AllowanceCheck, HttpError>>>,
    error: Option<String>
}

impl Page for Setup {
    fn page(&mut self, ui: &mut eframe::egui::Ui, state: &mut StateData) {
        ui.label("Welcome to the unofficial Eskom-Se-Push Notification Desktop App");
        ui.label("This app will notify you before load sheeding starts in your area.");
        ui.label("However, there will be some limitations as follows:");
        ui.label("-> The notifications will not be live. It will check the status every 30 min at the very least.");
        ui.label("-> You will require an API key from Eskom-se-Push. The link is provided below.");
        // TODO update URL
        ui.hyperlink("https://www.egui.rs/");

        ui.separator();
        ui.add_enabled_ui(!self.testing, |ui| {
            ui.label("Enter your API key here:");
            ui.text_edit_singleline(&mut self.api_key);
        });
        ui.add_enabled_ui(!self.api_key.is_empty(), |ui| {
            if !self.testing && ui.button("Test").clicked() {
                self.error = None;
                self.testing = true;
                // TODO test the API key
                let api_key = self.api_key.clone();
                self.thread = Some(thread::spawn(move || {
                    let t = AllowanceCheckURLBuilder::default().build().unwrap();
                    t.reqwest(api_key.as_str())
                }));
            } else {
                ui.spinner();
                
            }
        });
        if let Some(err_msg) = &self.error {
            ui.colored_label(Color32::RED, err_msg);
        }
        if self.thread.is_some() {
            if let Some(thr) = self.thread.take() {
                if thr.is_finished() {
                    let t = thr.join();
                    match t {
                        Ok(resp) => match resp {
                            Ok(_result) => {
                                state.page = ActivePage::Home;
                                state.api_key = self.api_key.clone();
                                self.api_key = String::new();
                            },
                            Err(err) => self.error = {
                                match err {
                                    HttpError::APIError(APIError::Forbidden) => Some("The API key is invalid.".to_owned()),
                                    HttpError::Timeout => Some("The API call timed out.".to_owned()),
                                    HttpError::NoInternet => Some("No internet access.".to_owned()),
                                    HttpError::Unknown => Some("An error occurred".to_owned()),
                                    HttpError::ResponseError(_) => Some("An error occurred".to_owned()),
                                    HttpError::APIError(_) => Some("An error occurred.".to_owned()),
                                    HttpError::UreqResponseError(_) => Some("An error occurred".to_owned()),
                                    HttpError::SearchTextNotSet => Some("An error occurred".to_owned()),
                                    HttpError::AreaIdNotSet => Some("An error occurred".to_owned()),
                                    HttpError::LongitudeOrLatitudeNotSet { longitude: _, latitude: _ } => Some("An error occurred".to_owned()),
                                    HttpError::UnknownError(_) => Some("An error occurred".to_owned()),
                                }
                            },
                        },
                        Err(e) => {
                            eprintln!("Error joining thread: {:?}", e);
                            self.error = Some(format!("Error joining thread: {:?}", e));
                        },
                    }
                } else {
                    self.thread = Some(thr);
                }
            }
        }
    }
}
