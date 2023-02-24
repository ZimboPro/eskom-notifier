use crate::{traits::Page, StateData};

#[derive(Debug, Default)]
pub struct HomePage {

}

impl Page for HomePage {
    fn page(&mut self, ui: &mut eframe::egui::Ui, _page: &mut StateData) {
        ui.label("HomePage");
    }
}