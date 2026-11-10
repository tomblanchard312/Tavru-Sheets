//! Tavru Sheets releases are separate from the upstream GridCraft update feed.
use crate::SheetApp;

pub(crate) fn start(app: &mut SheetApp, manual: bool) {
    if manual {
        app.message =
            Some(("Tavru Sheets updates".into(), "Get Tavru Sheets releases at https://github.com/tomblanchard312/Tavru-Sheets/releases".into()));
    }
}
pub(crate) fn tick(_: &mut SheetApp, _: &egui::Context) {}
pub(crate) fn show(_: &mut SheetApp, _: &egui::Context) {}
