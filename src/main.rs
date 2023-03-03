mod area_details_page;
mod cache_handler;
mod find_area_page;
mod helpers;
mod home_page;
mod layouts;
mod settings_page;
mod setup;
mod traits;

use std::ops::{Index, IndexMut};

use cache_handler::{read_cache, save_contents, save_state};
use directories_next::ProjectDirs;
use eframe::{egui::CentralPanel, run_native, App, NativeOptions};
use eskom_se_push_api::area_info::AreaInfo;

use serde::{Deserialize, Serialize};

use traits::Page;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivePage {
  Home,
  Setup,
  FindArea,
  AreaDetails(String),
  Settings,
}

impl Index<ActivePage> for Vec<Box<dyn Page>> {
  type Output = Box<dyn Page>;

  fn index(&self, index: ActivePage) -> &Self::Output {
    match index {
      ActivePage::Home => self.index(0),
      ActivePage::Setup => self.index(1),
      ActivePage::FindArea => self.index(2),
      ActivePage::AreaDetails(_) => self.index(3),
      ActivePage::Settings => self.index(4),
    }
  }
}

impl IndexMut<ActivePage> for Vec<Box<dyn Page>> {
  fn index_mut(&mut self, index: ActivePage) -> &mut Self::Output {
    match index {
      ActivePage::Home => self.index_mut(0),
      ActivePage::Setup => self.index_mut(1),
      ActivePage::FindArea => self.index_mut(2),
      ActivePage::AreaDetails(_) => self.index_mut(3),
      ActivePage::Settings => self.index_mut(4),
    }
  }
}

impl Default for ActivePage {
  fn default() -> Self {
    ActivePage::Home
  }
}

#[derive(Default)]
struct EskomApp {
  state: StateData,
  // client:
  pages: Vec<Box<dyn Page>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct StateData {
  ids: Vec<SelectedAreaInfo>,
  page: ActivePage,
  api_key: String,
}

impl StateData {
  pub fn update_cache(&self) {
    save_state(&self);
  }
}

const CONFIG_FILE: &str = "eskom-notifier.yaml";

impl EskomApp {
  pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
    let mut app = Self {
      state: StateData::default(),
      pages: vec![
        Box::<home_page::HomePage>::default(),
        Box::<setup::Setup>::default(),
        Box::new(find_area_page::FindAreaPage::new()),
        Box::<area_details_page::AreaDetailsPage>::default(),
        Box::<settings_page::SettingsPage>::default(),
      ],
    };
    app.read_cache();
    if app.state.api_key.is_empty() {
      app.state.page = ActivePage::Setup
    }
    app
  }

  pub fn read_cache(&mut self) {
    match read_cache() {
      Ok(state) => self.state = state,
      Err(e) => eprintln!("Error getting configuration: {}", e),
    }
  }
}

impl App for EskomApp {
  fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    CentralPanel::default().show(ctx, |ui| {
      self.pages[self.state.page.clone()].page(ui, &mut self.state);
    });
  }

  fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    if let Some(config) = ProjectDirs::from("io", "South Africa", "Eskom Notifier") {
      let t = config.config_dir().join(CONFIG_FILE);
      self.state.page = ActivePage::Home;
      if let Err(e) = save_contents(&t, &self.state) {
        eprintln!("Saving state error: {}", e)
      }
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SelectedAreaInfo {
  id: String,
  details: AreaInfo,
}

fn main() {
  let options = NativeOptions::default();
  match run_native(
    "Eskom Notifier",
    options,
    Box::new(|cc| Box::new(EskomApp::new(cc))),
  ) {
    Ok(_) => {}
    Err(err) => eprintln!("Error: {}", err),
  }
}
