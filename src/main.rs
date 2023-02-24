mod setup;
mod traits;
mod home;

use std::ops::{Index, IndexMut};

use eframe::{App, egui::{CentralPanel, Ui}};
// use eskom_se_push_api::
use eskom_se_push_api::area_info::AreaInfo;
use home::HomePage;
use setup::Setup;
use traits::Page;

#[derive(Debug, Clone, Copy)]
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

#[derive(Default)]
pub struct StateData {
    ids: Vec<SelectedAreaInfo>,
    page: ActivePage,
    api_key: String,
}

impl EskomApp{
    pub fn new() -> Self {
        Self {
            state: StateData::default(),
            pages: vec![
                Box::new(HomePage::default()),
                Box::new(Setup::default())
                ],
        }
    }
}

impl App for EskomApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.pages[self.state.page].page(ui, &mut self.state);
        });
    }
}

#[derive(Debug, Default, Clone)]
struct SelectedAreaInfo {
    id: String,
    details: AreaInfo
}


fn main() {
    
}
