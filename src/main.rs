use eframe::NativeOptions;
use egui::ViewportBuilder;

use crate::{app::GraphApp, ui::Theme};

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
        Box::new(|cc| {
            Theme::new().apply(&cc.egui_ctx);
            Ok(Box::<GraphApp>::default())
        }),
    )
}
