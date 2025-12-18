use crate::app::GraphApp;

pub fn render(ctx: &egui::Context, app: &mut GraphApp) {
    let header = app.year.to_string();
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(header);
    });
}
