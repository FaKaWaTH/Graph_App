use eframe::NativeOptions;
use egui::ViewportBuilder;

use crate::app::GraphApp;

mod app;
mod ui;

fn main() -> eframe::Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Graph App",
        options,
        Box::new(|_cc| Ok(Box::<GraphApp>::default())),
    )
}
