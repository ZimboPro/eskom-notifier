mod setup;
mod traits;
mod home_page;
mod cache_handler;

use std::ops::{Index, IndexMut};

use cache_handler::load_file_and_deserialise;
use directories_next::ProjectDirs;
use eframe::{App, egui::{CentralPanel, Ui}, NativeOptions, run_native};
// use eskom_se_push_api::
use eskom_se_push_api::area_info::AreaInfo;
use home_page::HomePage;
use serde::{Deserialize, Serialize};
use setup::Setup;
use traits::Page;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActivePage {
    Home,
    Setup,
    FindArea,
    AreaDetails,
    Settings
}

impl Index<ActivePage> for Vec<Box<dyn Page>> {
    type Output = Box<dyn Page>;

    fn index(&self, index: ActivePage) -> &Self::Output {
        match index {
            ActivePage::Home => &self.index(0),
            ActivePage::Setup => &self.index(1),
            ActivePage::FindArea => &self.index(2),
            ActivePage::AreaDetails => &self.index(3),
            ActivePage::Settings => &self.index(4),
        }
    }
}

impl IndexMut<ActivePage> for Vec<Box<dyn Page>> {
    fn index_mut(&mut self, index: ActivePage) -> &mut Self::Output {
        match index {
            ActivePage::Home => self.index_mut(0),
            ActivePage::Setup => self.index_mut(1),
            ActivePage::FindArea => self.index_mut(2),
            ActivePage::AreaDetails => self.index_mut(3),
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

const CONFIG_FILE: &str = "eskom-notifier.yaml";

impl EskomApp{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            state: StateData::default(),
            pages: vec![
                Box::new(HomePage::default()),
                Box::new(Setup::default())
                ],
        };
        app.read_cache();
        app
    }

    pub fn read_cache(&mut self) {
       if let Some(config) = ProjectDirs::from("io", "South Africa",  "Eskom Notifier") {
        let t = config.config_dir().join(CONFIG_FILE);
        if t.is_file() {
            match load_file_and_deserialise(&t) {
                Ok(state) => self.state = state,
                Err(e) => eprintln!("Error getting configuration: {}", e),
            }
        }
       }
    }
}

impl App for EskomApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.pages[self.state.page].page(ui, &mut self.state);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct SelectedAreaInfo {
    id: String,
    details: AreaInfo
}


fn main() {
    let options = NativeOptions::default();
    run_native(
        "Eskom Notifier",
        options,
        Box::new(|cc| Box::new(EskomApp::new(cc))));
}
